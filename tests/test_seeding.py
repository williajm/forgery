"""Tests for deterministic seeding."""

from forgery import Faker, emails, integers, names, seed, uuids


class TestModuleLevelSeeding:
    """Tests for module-level seed() function."""

    def test_same_seed_same_names(self) -> None:
        """Same seed should produce same names."""
        seed(42)
        names1 = names(100)

        seed(42)
        names2 = names(100)

        assert names1 == names2

    def test_same_seed_same_emails(self) -> None:
        """Same seed should produce same emails."""
        seed(42)
        emails1 = emails(100)

        seed(42)
        emails2 = emails(100)

        assert emails1 == emails2

    def test_same_seed_same_integers(self) -> None:
        """Same seed should produce same integers."""
        seed(42)
        ints1 = integers(100, 0, 1000)

        seed(42)
        ints2 = integers(100, 0, 1000)

        assert ints1 == ints2

    def test_same_seed_same_uuids(self) -> None:
        """Same seed should produce same UUIDs."""
        seed(42)
        uuids1 = uuids(100)

        seed(42)
        uuids2 = uuids(100)

        assert uuids1 == uuids2

    def test_different_seeds_different_output(self) -> None:
        """Different seeds should produce different output."""
        seed(42)
        names1 = names(100)

        seed(43)
        names2 = names(100)

        assert names1 != names2

    def test_seed_affects_subsequent_calls(self) -> None:
        """Seeding should affect all subsequent calls in order."""
        seed(42)
        names1 = names(50)
        emails1 = emails(50)
        ints1 = integers(50, 0, 100)

        seed(42)
        names2 = names(50)
        emails2 = emails(50)
        ints2 = integers(50, 0, 100)

        assert names1 == names2
        assert emails1 == emails2
        assert ints1 == ints2


class TestFakerInstanceSeeding:
    """Tests for Faker instance-level seeding."""

    def test_instance_seed_determinism(self) -> None:
        """Instance seeding should be deterministic."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        assert fake1.names(100) == fake2.names(100)

    def test_instance_independence(self) -> None:
        """Different instances should be independent."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(99)

        # Generate from both - they should differ
        names1 = fake1.names(100)
        names2 = fake2.names(100)

        assert names1 != names2

    def test_instance_does_not_affect_module(self) -> None:
        """Instance seeding should not affect module-level generator."""
        # Seed the module
        seed(42)
        module_names1 = names(50)

        # Create and seed an instance differently
        fake = Faker()
        fake.seed(99)
        _ = fake.names(50)

        # Re-seed module and check it wasn't affected
        seed(42)
        module_names2 = names(50)

        assert module_names1 == module_names2

    def test_multiple_instances_independent(self) -> None:
        """Multiple instances should not interfere with each other."""
        fake1 = Faker()
        fake2 = Faker()
        fake3 = Faker()

        fake1.seed(1)
        fake2.seed(2)
        fake3.seed(1)  # Same as fake1

        names1 = fake1.names(50)
        names2 = fake2.names(50)
        names3 = fake3.names(50)

        assert names1 == names3  # Same seed
        assert names1 != names2  # Different seed


class TestSeedEdgeCases:
    """Tests for seeding edge cases."""

    def test_seed_zero(self) -> None:
        """Seed of 0 should work."""
        seed(0)
        result1 = names(10)

        seed(0)
        result2 = names(10)

        assert result1 == result2

    def test_seed_large_value(self) -> None:
        """Large seed values should work."""
        large_seed = 2**63 - 1  # Max i64
        seed(large_seed)
        result1 = names(10)

        seed(large_seed)
        result2 = names(10)

        assert result1 == result2

    def test_reseed_resets_state(self) -> None:
        """Re-seeding should reset the RNG state."""
        seed(42)
        names(100)  # Advance state
        emails(100)  # Advance more

        seed(42)  # Reset
        fresh_names = names(100)

        seed(42)
        expected_names = names(100)

        assert fresh_names == expected_names
