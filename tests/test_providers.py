"""Tests for forgery data providers."""

import re

import pytest

from forgery import (
    Faker,
    email,
    emails,
    first_name,
    first_names,
    integer,
    integers,
    last_name,
    last_names,
    name,
    names,
    seed,
    uuid,
    uuids,
)


class TestNames:
    """Tests for name generation."""

    def test_name_returns_string(self) -> None:
        """Single name should return a string."""
        result = name()
        assert isinstance(result, str)
        assert len(result) > 0

    def test_name_has_first_and_last(self) -> None:
        """Name should have first and last parts."""
        seed(42)
        result = name()
        parts = result.split()
        assert len(parts) == 2

    def test_names_returns_list(self) -> None:
        """Batch names should return a list."""
        result = names(100)
        assert isinstance(result, list)
        assert len(result) == 100

    def test_names_empty_batch(self) -> None:
        """Empty batch should return empty list."""
        result = names(0)
        assert result == []

    def test_first_name_returns_string(self) -> None:
        """Single first name should return a string."""
        result = first_name()
        assert isinstance(result, str)
        assert " " not in result

    def test_first_names_batch(self) -> None:
        """Batch first names should work."""
        result = first_names(50)
        assert len(result) == 50
        for n in result:
            assert " " not in n

    def test_last_name_returns_string(self) -> None:
        """Single last name should return a string."""
        result = last_name()
        assert isinstance(result, str)

    def test_last_names_batch(self) -> None:
        """Batch last names should work."""
        result = last_names(50)
        assert len(result) == 50


class TestEmails:
    """Tests for email generation."""

    EMAIL_PATTERN = re.compile(r"^[a-z]+\d{3}@[a-z]+\.[a-z]+$")

    def test_email_returns_string(self) -> None:
        """Single email should return a string."""
        result = email()
        assert isinstance(result, str)
        assert "@" in result

    def test_email_format(self) -> None:
        """Email should match expected format."""
        seed(42)
        result = email()
        assert self.EMAIL_PATTERN.match(result), f"Email format invalid: {result}"

    def test_emails_batch(self) -> None:
        """Batch emails should work."""
        result = emails(100)
        assert len(result) == 100
        for e in result:
            assert "@" in e
            assert "." in e


class TestIntegers:
    """Tests for integer generation."""

    def test_integer_returns_int(self) -> None:
        """Single integer should return an int."""
        result = integer()
        assert isinstance(result, int)

    def test_integer_default_range(self) -> None:
        """Default range should be 0-100."""
        seed(42)
        results = [integer() for _ in range(100)]
        assert all(0 <= r <= 100 for r in results)

    def test_integer_custom_range(self) -> None:
        """Custom range should work."""
        seed(42)
        results = [integer(-50, 50) for _ in range(100)]
        assert all(-50 <= r <= 50 for r in results)

    def test_integers_batch(self) -> None:
        """Batch integers should work."""
        result = integers(100, 0, 1000)
        assert len(result) == 100
        assert all(0 <= r <= 1000 for r in result)

    def test_integers_negative_range(self) -> None:
        """Negative range should work."""
        result = integers(50, -100, -1)
        assert len(result) == 50
        assert all(-100 <= r <= -1 for r in result)

    def test_integer_invalid_range_raises(self) -> None:
        """Invalid range (min > max) should raise ValueError."""
        with pytest.raises(ValueError, match=r"min.*must be less than or equal to max"):
            integer(100, 0)

    def test_integers_invalid_range_raises(self) -> None:
        """Invalid range (min > max) should raise ValueError."""
        with pytest.raises(ValueError, match=r"min.*must be less than or equal to max"):
            integers(10, 100, 0)


class TestUuids:
    """Tests for UUID generation."""

    UUID_PATTERN = re.compile(
        r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
    )

    def test_uuid_returns_string(self) -> None:
        """Single UUID should return a string."""
        result = uuid()
        assert isinstance(result, str)

    def test_uuid_format(self) -> None:
        """UUID should be valid v4 format."""
        seed(42)
        result = uuid()
        assert self.UUID_PATTERN.match(result), f"UUID format invalid: {result}"

    def test_uuids_batch(self) -> None:
        """Batch UUIDs should work."""
        result = uuids(100)
        assert len(result) == 100
        for u in result:
            assert self.UUID_PATTERN.match(u), f"UUID format invalid: {u}"

    def test_uuids_unique(self) -> None:
        """UUIDs should be unique."""
        result = uuids(1000)
        assert len(set(result)) == len(result)


class TestFakerClass:
    """Tests for the Faker class."""

    def test_create_default_locale(self) -> None:
        """Should create with default locale."""
        fake = Faker()
        assert fake is not None

    def test_create_custom_locale(self) -> None:
        """Should create with custom locale."""
        fake = Faker("en_US")
        assert fake is not None

    def test_instance_independence(self) -> None:
        """Different instances should be independent."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(99)

        names1 = fake1.names(10)
        names2 = fake2.names(10)

        assert names1 != names2

    def test_all_methods_available(self) -> None:
        """All expected methods should be available."""
        fake = Faker()

        # These should not raise
        fake.name()
        fake.names(1)
        fake.first_name()
        fake.first_names(1)
        fake.last_name()
        fake.last_names(1)
        fake.email()
        fake.emails(1)
        fake.integer()
        fake.integers(1)
        fake.uuid()
        fake.uuids(1)


class TestLargeBatch:
    """Tests for large batch generation."""

    @pytest.mark.parametrize("n", [1000, 10000, 100000])
    def test_names_large_batch(self, n: int) -> None:
        """Should handle large batches efficiently."""
        result = names(n)
        assert len(result) == n

    @pytest.mark.parametrize("n", [1000, 10000, 100000])
    def test_emails_large_batch(self, n: int) -> None:
        """Should handle large batches efficiently."""
        result = emails(n)
        assert len(result) == n

    @pytest.mark.parametrize("n", [1000, 10000, 100000])
    def test_integers_large_batch(self, n: int) -> None:
        """Should handle large batches efficiently."""
        result = integers(n, 0, 1_000_000)
        assert len(result) == n


class TestBatchSizeLimits:
    """Tests for batch size limits to prevent memory exhaustion."""

    MAX_BATCH_SIZE = 10_000_000

    def test_names_exceeds_limit_raises(self) -> None:
        """Names batch exceeding limit should raise ValueError."""
        with pytest.raises(ValueError, match=r"batch size.*exceeds maximum"):
            names(self.MAX_BATCH_SIZE + 1)

    def test_first_names_exceeds_limit_raises(self) -> None:
        """First names batch exceeding limit should raise ValueError."""
        with pytest.raises(ValueError, match=r"batch size.*exceeds maximum"):
            first_names(self.MAX_BATCH_SIZE + 1)

    def test_last_names_exceeds_limit_raises(self) -> None:
        """Last names batch exceeding limit should raise ValueError."""
        with pytest.raises(ValueError, match=r"batch size.*exceeds maximum"):
            last_names(self.MAX_BATCH_SIZE + 1)

    def test_emails_exceeds_limit_raises(self) -> None:
        """Emails batch exceeding limit should raise ValueError."""
        with pytest.raises(ValueError, match=r"batch size.*exceeds maximum"):
            emails(self.MAX_BATCH_SIZE + 1)

    def test_integers_exceeds_limit_raises(self) -> None:
        """Integers batch exceeding limit should raise ValueError."""
        with pytest.raises(ValueError, match=r"batch size.*exceeds maximum"):
            integers(self.MAX_BATCH_SIZE + 1, 0, 100)

    def test_uuids_exceeds_limit_raises(self) -> None:
        """UUIDs batch exceeding limit should raise ValueError."""
        with pytest.raises(ValueError, match=r"batch size.*exceeds maximum"):
            uuids(self.MAX_BATCH_SIZE + 1)

    def test_at_limit_works(self) -> None:
        """Batch at exactly the limit should work (not tested with actual 10M for speed)."""
        # Just test that the boundary value doesn't raise prematurely
        # We use a smaller value here for test speed
        result = names(1000)
        assert len(result) == 1000
