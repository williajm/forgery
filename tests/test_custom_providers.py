"""Tests for custom providers API (Phase 3.2)."""

import pytest

from forgery import (
    Faker,
    add_provider,
    add_weighted_provider,
    generate,
    generate_batch,
    has_provider,
    records,
    remove_provider,
    seed,
)


class TestCustomProviderRegistration:
    """Tests for provider registration."""

    def test_add_provider_basic(self) -> None:
        """Basic provider registration should work."""
        f = Faker()
        f.add_provider("department", ["Engineering", "Sales", "HR"])
        assert f.has_provider("department")

    def test_add_provider_builtin_name_fails(self) -> None:
        """Cannot use built-in type names."""
        f = Faker()
        with pytest.raises(ValueError, match="conflicts with built-in"):
            f.add_provider("name", ["a", "b"])

    def test_add_provider_empty_options_fails(self) -> None:
        """Cannot register with empty options."""
        f = Faker()
        with pytest.raises(ValueError, match="empty"):
            f.add_provider("department", [])

    def test_add_weighted_provider(self) -> None:
        """Weighted provider registration should work."""
        f = Faker()
        f.add_weighted_provider("status", [("active", 80), ("inactive", 20)])
        assert f.has_provider("status")

    def test_add_weighted_provider_all_zero_fails(self) -> None:
        """Cannot register with all zero weights."""
        f = Faker()
        with pytest.raises(ValueError, match="zero"):
            f.add_weighted_provider("status", [("a", 0), ("b", 0)])

    def test_remove_provider(self) -> None:
        """Provider removal should work."""
        f = Faker()
        f.add_provider("department", ["Engineering", "Sales"])
        assert f.remove_provider("department") is True
        assert f.has_provider("department") is False
        assert f.remove_provider("department") is False  # Already removed

    def test_list_providers(self) -> None:
        """List providers should return all registered names."""
        f = Faker()
        f.add_provider("dept", ["a"])
        f.add_provider("status", ["x"])
        providers = f.list_providers()
        assert "dept" in providers
        assert "status" in providers


class TestCustomProviderGeneration:
    """Tests for generating values from custom providers."""

    def test_generate_single(self) -> None:
        """Generate single value should work."""
        f = Faker()
        f.seed(42)
        f.add_provider("department", ["Engineering", "Sales", "HR"])
        value = f.generate("department")
        assert value in ["Engineering", "Sales", "HR"]

    def test_generate_not_found_fails(self) -> None:
        """Generate for non-existent provider should fail."""
        f = Faker()
        with pytest.raises(ValueError, match="not found"):
            f.generate("nonexistent")

    def test_generate_batch(self) -> None:
        """Generate batch should work."""
        f = Faker()
        f.seed(42)
        f.add_provider("department", ["Engineering", "Sales", "HR"])
        values = f.generate_batch("department", 100)
        assert len(values) == 100
        for v in values:
            assert v in ["Engineering", "Sales", "HR"]

    def test_generate_deterministic(self) -> None:
        """Same seed should produce same results."""
        f1 = Faker()
        f2 = Faker()

        f1.add_provider("dept", ["a", "b", "c"])
        f2.add_provider("dept", ["a", "b", "c"])

        f1.seed(42)
        f2.seed(42)

        v1 = f1.generate_batch("dept", 100)
        v2 = f2.generate_batch("dept", 100)

        assert v1 == v2

    def test_weighted_distribution(self) -> None:
        """Weighted provider should follow distribution."""
        f = Faker()
        f.seed(42)
        f.add_weighted_provider("status", [("active", 90), ("inactive", 10)])

        values = f.generate_batch("status", 10000)
        active_count = values.count("active")

        # Should be roughly 90% (9000), allow some variance
        assert 8500 < active_count < 9500


class TestCustomProvidersInRecords:
    """Tests for custom providers in records() schema."""

    def test_records_with_custom_provider(self) -> None:
        """Custom providers should work in records schema."""
        f = Faker()
        f.seed(42)
        f.add_provider("department", ["Engineering", "Sales", "HR"])

        result = f.records(
            10,
            {
                "name": "name",
                "dept": "department",  # Custom provider
            },
        )

        assert len(result) == 10
        for row in result:
            assert "name" in row
            assert "dept" in row
            assert row["dept"] in ["Engineering", "Sales", "HR"]

    def test_records_tuples_with_custom_provider(self) -> None:
        """Custom providers should work in records_tuples schema."""
        f = Faker()
        f.seed(42)
        f.add_provider("status", ["active", "inactive"])

        result = f.records_tuples(
            10,
            {
                "id": "uuid",
                "status": "status",  # Custom provider
            },
        )

        assert len(result) == 10
        for row in result:
            assert len(row) == 2

    def test_records_unknown_type_fails(self) -> None:
        """Unknown type (not builtin, not custom) should fail."""
        f = Faker()
        with pytest.raises(ValueError, match="Unknown type"):
            f.records(1, {"field": "nonexistent_provider"})

    def test_records_with_mixed_providers(self) -> None:
        """Mix of built-in and custom providers should work."""
        f = Faker()
        f.seed(42)
        f.add_provider("department", ["Eng", "Sales"])
        f.add_weighted_provider("priority", [("high", 20), ("medium", 50), ("low", 30)])

        result = f.records(
            100,
            {
                "id": "uuid",
                "name": "name",
                "email": "email",
                "department": "department",
                "priority": "priority",
            },
        )

        assert len(result) == 100
        depts = {row["department"] for row in result}
        assert depts == {"Eng", "Sales"}


class TestModuleLevelConvenience:
    """Tests for module-level convenience functions."""

    def test_module_level_add_and_generate(self) -> None:
        """Module-level functions should work with default faker."""
        # Note: Uses global 'fake' instance
        seed(42)

        # Clean up any previous test state
        if has_provider("test_dept"):
            remove_provider("test_dept")

        add_provider("test_dept", ["A", "B", "C"])
        assert has_provider("test_dept")

        value = generate("test_dept")
        assert value in ["A", "B", "C"]

        values = generate_batch("test_dept", 50)
        assert len(values) == 50

        # Clean up
        remove_provider("test_dept")

    def test_module_level_weighted_provider(self) -> None:
        """Module-level weighted provider should work."""
        seed(42)

        if has_provider("test_status"):
            remove_provider("test_status")

        add_weighted_provider("test_status", [("yes", 90), ("no", 10)])
        assert has_provider("test_status")

        values = generate_batch("test_status", 1000)
        yes_count = values.count("yes")
        # Should be roughly 90% (900)
        assert 800 < yes_count < 1000

        # Clean up
        remove_provider("test_status")

    def test_module_level_records_with_custom(self) -> None:
        """Module-level records should work with custom providers."""
        seed(42)

        if has_provider("test_color"):
            remove_provider("test_color")

        add_provider("test_color", ["red", "green", "blue"])

        result = records(
            10,
            {
                "id": "uuid",
                "color": "test_color",
            },
        )

        assert len(result) == 10
        for row in result:
            assert row["color"] in ["red", "green", "blue"]

        # Clean up
        remove_provider("test_color")

    def test_module_level_list_providers(self) -> None:
        """Module-level list_providers should return sorted list."""
        from forgery import list_providers

        # Clean up any previous state
        for name in list_providers():
            remove_provider(name)

        # Add some providers in non-alphabetical order
        add_provider("zebra", ["z"])
        add_provider("alpha", ["a"])
        add_provider("middle", ["m"])

        providers = list_providers()
        assert providers == ["alpha", "middle", "zebra"]  # Should be sorted

        # Clean up
        for name in providers:
            remove_provider(name)

    def test_module_level_negative_weight_fails(self) -> None:
        """Module-level add_weighted_provider should reject negative weights."""
        with pytest.raises(ValueError, match="must be non-negative"):
            add_weighted_provider("bad_provider", [("good", 10), ("bad", -5)])


class TestEdgeCases:
    """Edge case tests."""

    def test_single_option_provider(self) -> None:
        """Provider with single option should always return it."""
        f = Faker()
        f.add_provider("singleton", ["only_option"])

        for _ in range(100):
            assert f.generate("singleton") == "only_option"

    def test_provider_independence(self) -> None:
        """Different Faker instances should have independent providers."""
        f1 = Faker()
        f2 = Faker()

        f1.add_provider("dept", ["A"])

        assert f1.has_provider("dept")
        assert not f2.has_provider("dept")

    def test_overwrite_provider(self) -> None:
        """Re-registering should overwrite previous provider."""
        f = Faker()
        f.add_provider("dept", ["A", "B"])
        f.add_provider("dept", ["X", "Y", "Z"])  # Overwrite

        f.seed(42)
        values = set(f.generate_batch("dept", 1000))
        assert values == {"X", "Y", "Z"}

    def test_weighted_zero_weight_items(self) -> None:
        """Zero-weight items should be excluded."""
        f = Faker()
        f.add_weighted_provider(
            "status",
            [
                ("active", 100),
                ("never", 0),  # Should never appear
            ],
        )

        f.seed(42)
        values = f.generate_batch("status", 1000)
        assert "never" not in values
        assert all(v == "active" for v in values)

    def test_empty_list_providers(self) -> None:
        """Empty list_providers should return empty list."""
        f = Faker()
        assert f.list_providers() == []

    def test_batch_size_validation(self) -> None:
        """Batch size should be validated."""
        f = Faker()
        f.add_provider("dept", ["A"])

        # Should work with reasonable batch size
        result = f.generate_batch("dept", 1000)
        assert len(result) == 1000

        # Batch size of 0 should return empty list
        result = f.generate_batch("dept", 0)
        assert result == []


class TestRecordsIntegrationDetails:
    """Detailed tests for records integration."""

    def test_custom_provider_in_records_deterministic(self) -> None:
        """Records with custom providers should be deterministic."""
        f1 = Faker()
        f2 = Faker()

        f1.add_provider("tier", ["gold", "silver", "bronze"])
        f2.add_provider("tier", ["gold", "silver", "bronze"])

        f1.seed(42)
        f2.seed(42)

        r1 = f1.records(100, {"tier": "tier", "name": "name"})
        r2 = f2.records(100, {"tier": "tier", "name": "name"})

        assert r1 == r2

    def test_custom_and_builtin_same_record(self) -> None:
        """Same record should mix custom and builtin consistently."""
        f = Faker()
        f.seed(42)
        f.add_provider("dept", ["A", "B"])

        results = f.records(
            50,
            {
                "name": "name",
                "email": "email",
                "dept": "dept",
                "age": ("int", 18, 65),
            },
        )

        # Check all fields are present and valid
        for row in results:
            assert isinstance(row["name"], str) and len(row["name"]) > 0
            assert "@" in row["email"]
            assert row["dept"] in ["A", "B"]
            assert 18 <= row["age"] <= 65

    def test_missing_custom_provider_fails_even_n_zero(self) -> None:
        """Missing custom provider should fail even when n=0."""
        f = Faker()
        # Don't register the provider, but reference it in schema
        # The error is "Unknown type" because schema parsing validates
        # that string types are either built-in or registered custom providers
        with pytest.raises(ValueError, match="Unknown type"):
            f.records(0, {"field": "nonexistent_provider"})
