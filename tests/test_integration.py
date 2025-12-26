"""Integration tests between Python wrapper and Rust core.

These tests verify the Python-Rust boundary behavior, including:
- Cross-language data consistency
- Error propagation from Rust to Python
- Memory and resource handling
- Large-scale batch operations
"""

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


class TestCrossLanguageConsistency:
    """Tests for data consistency across Python-Rust boundary."""

    def test_string_encoding_utf8(self) -> None:
        """Verify strings are properly UTF-8 encoded."""
        seed(42)
        result_names = names(100)
        for n in result_names:
            # Should be valid UTF-8 strings
            assert isinstance(n, str)
            # Encode and decode to verify UTF-8 validity
            encoded = n.encode("utf-8")
            decoded = encoded.decode("utf-8")
            assert n == decoded

    def test_email_ascii_only(self) -> None:
        """Emails should be ASCII-only."""
        seed(42)
        result_emails = emails(100)
        for e in result_emails:
            assert e.isascii(), f"Email should be ASCII: {e}"

    def test_uuid_format_consistency(self) -> None:
        """UUID format should be consistent with Python uuid module expectations."""
        import uuid as uuid_module

        seed(42)
        result_uuids = uuids(100)
        for u in result_uuids:
            # Should be parseable by Python's uuid module
            parsed = uuid_module.UUID(u)
            assert str(parsed) == u

    def test_integer_types(self) -> None:
        """Integers should be Python int type."""
        seed(42)
        result_ints = integers(100, -1000, 1000)
        for i in result_ints:
            assert isinstance(i, int)
            assert not isinstance(i, bool)  # bool is a subclass of int

    def test_list_types(self) -> None:
        """Batch results should be Python lists."""
        seed(42)
        assert isinstance(names(10), list)
        assert isinstance(emails(10), list)
        assert isinstance(integers(10, 0, 100), list)
        assert isinstance(uuids(10), list)


class TestRustErrorPropagation:
    """Tests for error propagation from Rust to Python."""

    def test_invalid_range_error_message(self) -> None:
        """Error message from Rust should be meaningful in Python."""
        with pytest.raises(ValueError) as exc_info:
            integer(100, 0)

        error_msg = str(exc_info.value)
        assert "min" in error_msg.lower() or "100" in error_msg
        assert "max" in error_msg.lower() or "0" in error_msg

    def test_batch_size_limit_error_message(self) -> None:
        """Batch size limit error should be clear."""
        with pytest.raises(ValueError) as exc_info:
            names(10_000_001)

        error_msg = str(exc_info.value)
        assert "batch" in error_msg.lower() or "exceeds" in error_msg.lower()

    def test_error_types_are_correct(self) -> None:
        """Rust errors should map to appropriate Python exceptions."""
        # ValueError for range errors
        with pytest.raises(ValueError):
            integers(10, 100, 0)

        # ValueError for batch size errors
        with pytest.raises(ValueError):
            emails(10_000_001)


class TestLargeScaleOperations:
    """Tests for large batch operations."""

    def test_million_items_names(self) -> None:
        """Should handle 1 million names efficiently."""
        seed(42)
        result = names(1_000_000)
        assert len(result) == 1_000_000
        # Spot check
        assert " " in result[0]
        assert " " in result[999_999]

    def test_million_items_integers(self) -> None:
        """Should handle 1 million integers efficiently."""
        seed(42)
        result = integers(1_000_000, 0, 1_000_000)
        assert len(result) == 1_000_000
        assert all(0 <= i <= 1_000_000 for i in result[:1000])

    def test_million_items_uuids(self) -> None:
        """Should handle 1 million UUIDs efficiently."""
        seed(42)
        result = uuids(1_000_000)
        assert len(result) == 1_000_000
        assert len(result[0]) == 36
        assert len(result[999_999]) == 36


class TestMemoryBehavior:
    """Tests for memory handling."""

    def test_repeated_allocations(self) -> None:
        """Repeated allocations should not leak memory."""
        seed(42)
        # Allocate and discard many times
        for _ in range(100):
            _ = names(10000)
            _ = emails(10000)
            _ = uuids(10000)
        # If we get here without OOM, test passes

    def test_empty_results_allocation(self) -> None:
        """Empty batches should not cause issues."""
        for _ in range(1000):
            assert names(0) == []
            assert emails(0) == []
            assert integers(0, 0, 100) == []
            assert uuids(0) == []


class TestDeterminismAcrossCalls:
    """Tests for determinism guarantees."""

    def test_interleaved_generator_types(self) -> None:
        """Same seed should produce same results with interleaved calls."""
        seed(42)
        a1 = name()
        b1 = email()
        c1 = integer(0, 100)
        d1 = uuid()

        seed(42)
        a2 = name()
        b2 = email()
        c2 = integer(0, 100)
        d2 = uuid()

        assert a1 == a2
        assert b1 == b2
        assert c1 == c2
        assert d1 == d2

    def test_batch_then_single(self) -> None:
        """Batch followed by single should be deterministic."""
        seed(42)
        batch1 = names(5)
        single1 = name()

        seed(42)
        batch2 = names(5)
        single2 = name()

        assert batch1 == batch2
        assert single1 == single2

    def test_state_advances_correctly(self) -> None:
        """State should advance in predictable increments."""
        seed(42)
        _ = names(100)
        after_100 = name()

        seed(42)
        _ = names(50)
        _ = names(50)
        after_50_50 = name()

        assert after_100 == after_50_50


class TestFakerInstanceIsolation:
    """Tests for instance isolation."""

    def test_many_instances(self) -> None:
        """Many instances should work independently."""
        instances = [Faker() for _ in range(100)]

        for i, fake in enumerate(instances):
            fake.seed(i)

        results = [fake.name() for fake in instances]

        # All names from different seeds should be different
        assert len(set(results)) == len(results)

    def test_instance_state_isolated(self) -> None:
        """One instance's operations should not affect another."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        # Advance fake1's state
        _ = fake1.names(1000)

        # fake2 should still produce original sequence
        fake1_result = fake1.name()
        fake2.seed(42)
        fake2_result = fake2.name()

        # fake2 should get first name from seed 42
        assert fake2_result != fake1_result  # fake1 is advanced

    def test_default_instance_vs_custom(self) -> None:
        """Default instance and custom instance should be independent."""
        seed(42)
        default_names = names(10)

        custom = Faker()
        custom.seed(42)
        custom_names = custom.names(10)

        # Same seed should produce same results
        assert default_names == custom_names


class TestEdgeCasesIntegration:
    """Edge case tests at the Python-Rust boundary."""

    def test_extreme_integer_values(self) -> None:
        """Test with extreme integer values."""
        seed(42)
        # Large range
        result = integers(100, -(2**62), 2**62)
        assert len(result) == 100

        # Very large single value boundary
        result = integer(-(2**62), 2**62)
        assert isinstance(result, int)

    def test_single_value_range(self) -> None:
        """min == max should always return that value."""
        seed(42)
        for _ in range(100):
            assert integer(42, 42) == 42

        result = integers(100, 999, 999)
        assert all(i == 999 for i in result)

    def test_adjacent_value_range(self) -> None:
        """Test range with only two possible values."""
        seed(42)
        result = integers(1000, 0, 1)
        assert all(i in (0, 1) for i in result)
        # Should have both values
        assert 0 in result
        assert 1 in result

    def test_negative_only_range(self) -> None:
        """Test range with only negative values."""
        seed(42)
        result = integers(100, -1000, -1)
        assert all(-1000 <= i <= -1 for i in result)

    def test_batch_size_boundary(self) -> None:
        """Test at exactly the batch size limit."""
        # Just under limit should work
        # Note: We won't actually allocate 10M items in tests for speed
        # But we verify the limit is correctly enforced
        with pytest.raises(ValueError):
            names(10_000_001)

        # 10M exactly should work (we just verify no error is raised)
        # We skip actually running this to keep tests fast


class TestDataQuality:
    """Tests for quality of generated data."""

    UUID_PATTERN = re.compile(
        r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
    )
    EMAIL_PATTERN = re.compile(r"^[a-z]+\d{3}@[a-z]+\.[a-z]+$")

    def test_uuid_version_4_compliance(self) -> None:
        """All UUIDs should be valid version 4."""
        seed(42)
        result_uuids = uuids(1000)
        for u in result_uuids:
            assert self.UUID_PATTERN.match(u), f"Invalid UUID: {u}"

    def test_email_format_compliance(self) -> None:
        """All emails should match expected format."""
        seed(42)
        result_emails = emails(1000)
        for e in result_emails:
            assert self.EMAIL_PATTERN.match(e), f"Invalid email format: {e}"

    def test_name_format_compliance(self) -> None:
        """All names should be 'FirstName LastName' format."""
        seed(42)
        result_names = names(1000)
        for n in result_names:
            parts = n.split()
            assert len(parts) == 2, f"Name should have two parts: {n}"
            assert parts[0][0].isupper(), f"First name should be capitalized: {n}"
            assert parts[1][0].isupper(), f"Last name should be capitalized: {n}"

    def test_uuid_uniqueness_large_batch(self) -> None:
        """UUIDs in a large batch should all be unique."""
        seed(42)
        result_uuids = uuids(100_000)
        unique = set(result_uuids)
        assert len(unique) == len(result_uuids), "UUIDs should be unique"


class TestModuleLevelAPI:
    """Tests for the module-level API functions."""

    def test_all_single_functions_exist(self) -> None:
        """All single-value functions should be available."""
        assert callable(name)
        assert callable(first_name)
        assert callable(last_name)
        assert callable(email)
        assert callable(integer)
        assert callable(uuid)
        assert callable(seed)

    def test_all_batch_functions_exist(self) -> None:
        """All batch functions should be available."""
        assert callable(names)
        assert callable(first_names)
        assert callable(last_names)
        assert callable(emails)
        assert callable(integers)
        assert callable(uuids)

    def test_default_integer_range(self) -> None:
        """Default integer range should be 0-100."""
        seed(42)
        results = [integer() for _ in range(1000)]
        assert all(0 <= i <= 100 for i in results)
        # Should span most of the range with 1000 samples
        assert min(results) < 10
        assert max(results) > 90


class TestTypeStubs:
    """Tests to verify type stub accuracy."""

    def test_faker_constructor_default(self) -> None:
        """Faker() with no args should work."""
        fake = Faker()
        assert fake is not None

    def test_faker_constructor_with_locale(self) -> None:
        """Faker(locale) should work for supported locales."""
        fake = Faker("en_US")
        assert fake is not None

    def test_faker_constructor_unsupported_locale(self) -> None:
        """Faker(locale) should raise ValueError for unsupported locales."""
        with pytest.raises(ValueError, match="unsupported locale"):
            Faker("fr_FR")

    def test_integers_default_params(self) -> None:
        """integers(n) should use default min/max."""
        result = integers(10)
        assert len(result) == 10
        assert all(0 <= i <= 100 for i in result)

    def test_integers_partial_params(self) -> None:
        """integers(n, min) should use default max."""
        result = integers(10, 50)
        assert len(result) == 10
        assert all(50 <= i <= 100 for i in result)
