"""Tests for batch generation consistency."""

from forgery import Faker, seed


class TestBatchSingleConsistency:
    """Tests that batch and single-value generation are consistent."""

    def test_names_batch_matches_singles(self) -> None:
        """Batch names should match sequential single calls."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        batch = fake1.names(10)
        singles = [fake2.name() for _ in range(10)]

        assert batch == singles

    def test_first_names_batch_matches_singles(self) -> None:
        """Batch first names should match sequential single calls."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        batch = fake1.first_names(10)
        singles = [fake2.first_name() for _ in range(10)]

        assert batch == singles

    def test_last_names_batch_matches_singles(self) -> None:
        """Batch last names should match sequential single calls."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        batch = fake1.last_names(10)
        singles = [fake2.last_name() for _ in range(10)]

        assert batch == singles

    def test_emails_batch_matches_singles(self) -> None:
        """Batch emails should match sequential single calls."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        batch = fake1.emails(10)
        singles = [fake2.email() for _ in range(10)]

        assert batch == singles

    def test_integers_batch_matches_singles(self) -> None:
        """Batch integers should match sequential single calls."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        batch = fake1.integers(10, 0, 100)
        singles = [fake2.integer(0, 100) for _ in range(10)]

        assert batch == singles

    def test_uuids_batch_matches_singles(self) -> None:
        """Batch UUIDs should match sequential single calls."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        batch = fake1.uuids(10)
        singles = [fake2.uuid() for _ in range(10)]

        assert batch == singles


class TestBatchPerformance:
    """Tests that batch operations are efficient."""

    def test_batch_returns_preallocated_list(self) -> None:
        """Batch should return a list of exact size."""
        seed(42)
        from forgery import names

        result = names(1000)
        assert len(result) == 1000

    def test_empty_batch_returns_empty_list(self) -> None:
        """Empty batch should return empty list."""
        from forgery import emails, integers, names, uuids

        assert names(0) == []
        assert emails(0) == []
        assert integers(0, 0, 100) == []
        assert uuids(0) == []
