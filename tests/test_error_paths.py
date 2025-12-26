"""Tests for error paths and exception handling.

These tests ensure proper error handling for:
- Invalid input parameters
- Boundary violations
- Resource limits
- Error message quality
"""

import contextlib

import pytest

from forgery import (
    Faker,
    emails,
    first_names,
    integer,
    integers,
    last_names,
    names,
    uuids,
)


class TestIntegerRangeErrors:
    """Tests for integer range validation errors."""

    def test_min_greater_than_max_single(self) -> None:
        """integer(min > max) should raise ValueError."""
        with pytest.raises(ValueError):
            integer(100, 0)

    def test_min_greater_than_max_batch(self) -> None:
        """integers(n, min > max) should raise ValueError."""
        with pytest.raises(ValueError):
            integers(10, 100, 0)

    def test_negative_min_greater_than_negative_max(self) -> None:
        """Negative ranges with min > max should raise."""
        with pytest.raises(ValueError):
            integer(-1, -100)

    def test_error_message_contains_values(self) -> None:
        """Error message should contain the invalid values."""
        with pytest.raises(ValueError) as exc_info:
            integer(500, 100)

        msg = str(exc_info.value).lower()
        # Should mention min/max or the actual values
        assert "500" in msg or "100" in msg or "min" in msg

    def test_various_invalid_ranges(self) -> None:
        """Test multiple invalid range combinations."""
        invalid_ranges = [
            (1, 0),
            (10, -10),
            (0, -1),
            (1000, 999),
            (-(2**30), -(2**31)),
        ]

        for min_val, max_val in invalid_ranges:
            with pytest.raises(ValueError):
                integer(min_val, max_val)


class TestBatchSizeLimitErrors:
    """Tests for batch size limit enforcement."""

    MAX_BATCH_SIZE = 10_000_000

    def test_names_exceeds_limit(self) -> None:
        """names(n > limit) should raise ValueError."""
        with pytest.raises(ValueError):
            names(self.MAX_BATCH_SIZE + 1)

    def test_first_names_exceeds_limit(self) -> None:
        """first_names(n > limit) should raise ValueError."""
        with pytest.raises(ValueError):
            first_names(self.MAX_BATCH_SIZE + 1)

    def test_last_names_exceeds_limit(self) -> None:
        """last_names(n > limit) should raise ValueError."""
        with pytest.raises(ValueError):
            last_names(self.MAX_BATCH_SIZE + 1)

    def test_emails_exceeds_limit(self) -> None:
        """emails(n > limit) should raise ValueError."""
        with pytest.raises(ValueError):
            emails(self.MAX_BATCH_SIZE + 1)

    def test_integers_exceeds_limit(self) -> None:
        """integers(n > limit) should raise ValueError."""
        with pytest.raises(ValueError):
            integers(self.MAX_BATCH_SIZE + 1, 0, 100)

    def test_uuids_exceeds_limit(self) -> None:
        """uuids(n > limit) should raise ValueError."""
        with pytest.raises(ValueError):
            uuids(self.MAX_BATCH_SIZE + 1)

    def test_batch_size_error_message_quality(self) -> None:
        """Error message should be descriptive."""
        with pytest.raises(ValueError) as exc_info:
            names(self.MAX_BATCH_SIZE + 100)

        msg = str(exc_info.value).lower()
        # Should mention batch size or maximum
        assert "batch" in msg or "maximum" in msg or "exceeds" in msg

    def test_at_limit_works(self) -> None:
        """Batch at exactly limit should work (tested with smaller value)."""
        # We don't actually test 10M for performance, but verify the check
        # Test with safe smaller value
        result = names(1000)
        assert len(result) == 1000

    def test_way_over_limit(self) -> None:
        """Extremely large batch size should raise."""
        with pytest.raises(ValueError):
            names(10**18)  # Absurdly large


class TestFakerInstanceErrors:
    """Tests for Faker instance method errors."""

    def test_faker_integer_range_error(self) -> None:
        """Faker.integer with invalid range should raise."""
        fake = Faker()
        with pytest.raises(ValueError):
            fake.integer(100, 0)

    def test_faker_integers_range_error(self) -> None:
        """Faker.integers with invalid range should raise."""
        fake = Faker()
        with pytest.raises(ValueError):
            fake.integers(10, 100, 0)

    def test_faker_batch_size_errors(self) -> None:
        """Faker batch methods should enforce size limits."""
        fake = Faker()
        max_size = 10_000_001

        with pytest.raises(ValueError):
            fake.names(max_size)

        with pytest.raises(ValueError):
            fake.emails(max_size)

        with pytest.raises(ValueError):
            fake.integers(max_size, 0, 100)

        with pytest.raises(ValueError):
            fake.uuids(max_size)


class TestErrorTypeConsistency:
    """Tests that error types are consistent."""

    def test_all_range_errors_are_value_errors(self) -> None:
        """All range violations should raise ValueError."""
        fake = Faker()

        # Module-level
        assert_raises_value_error(lambda: integer(10, 0))
        assert_raises_value_error(lambda: integers(10, 10, 0))

        # Instance methods
        assert_raises_value_error(lambda: fake.integer(10, 0))
        assert_raises_value_error(lambda: fake.integers(10, 10, 0))

    def test_all_batch_size_errors_are_value_errors(self) -> None:
        """All batch size violations should raise ValueError."""
        fake = Faker()
        big = 10_000_001

        # Module-level
        assert_raises_value_error(lambda: names(big))
        assert_raises_value_error(lambda: emails(big))
        assert_raises_value_error(lambda: integers(big, 0, 100))
        assert_raises_value_error(lambda: uuids(big))

        # Instance methods
        assert_raises_value_error(lambda: fake.names(big))
        assert_raises_value_error(lambda: fake.emails(big))
        assert_raises_value_error(lambda: fake.integers(big, 0, 100))
        assert_raises_value_error(lambda: fake.uuids(big))


class TestBoundaryConditions:
    """Tests for boundary condition handling."""

    def test_zero_batch_size(self) -> None:
        """Zero batch size should return empty list, not error."""
        assert names(0) == []
        assert emails(0) == []
        assert integers(0, 0, 100) == []
        assert uuids(0) == []

    def test_one_item_batch(self) -> None:
        """Single item batch should work."""
        assert len(names(1)) == 1
        assert len(emails(1)) == 1
        assert len(integers(1, 0, 100)) == 1
        assert len(uuids(1)) == 1

    def test_min_equals_max(self) -> None:
        """min == max should return that value, not error."""
        result = integer(42, 42)
        assert result == 42

        results = integers(100, 42, 42)
        assert all(r == 42 for r in results)


class TestRecoverability:
    """Tests that errors don't corrupt state."""

    def test_error_does_not_corrupt_faker_state(self) -> None:
        """After an error, Faker should still be usable."""
        fake = Faker()
        fake.seed(42)

        # Cause an error
        with contextlib.suppress(ValueError):
            fake.integer(100, 0)

        # Should still work
        result = fake.name()
        assert isinstance(result, str)
        assert " " in result

    def test_error_does_not_corrupt_module_state(self) -> None:
        """After an error, module functions should still work."""
        from forgery import name, seed

        seed(42)

        # Cause an error
        with contextlib.suppress(ValueError):
            integer(100, 0)

        # Should still work
        result = name()
        assert isinstance(result, str)

    def test_multiple_errors_dont_accumulate(self) -> None:
        """Multiple errors should not cause cumulative issues."""
        for _ in range(100):
            with contextlib.suppress(ValueError):
                integer(100, 0)

        # Should still work fine
        assert isinstance(integer(0, 100), int)


def assert_raises_value_error(func: callable) -> None:
    """Helper to assert ValueError is raised."""
    try:
        func()
        raise AssertionError("Expected ValueError was not raised")
    except ValueError:
        pass  # Expected
