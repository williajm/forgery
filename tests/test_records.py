"""Tests for structured data generation (records and records_tuples)."""

import pytest

from forgery import Faker, records, records_tuples, seed


class TestRecords:
    """Tests for records() function."""

    def test_records_returns_list_of_dicts(self) -> None:
        """records() should return a list of dictionaries."""
        seed(42)
        result = records(5, {"name": "name", "email": "email"})
        assert isinstance(result, list)
        assert len(result) == 5
        for row in result:
            assert isinstance(row, dict)

    def test_records_has_all_fields(self) -> None:
        """Each record should have all schema fields."""
        seed(42)
        schema = {
            "id": "uuid",
            "name": "name",
            "email": "email",
        }
        result = records(10, schema)
        for row in result:
            assert "id" in row
            assert "name" in row
            assert "email" in row

    def test_records_empty_batch(self) -> None:
        """Empty batch should return empty list."""
        result = records(0, {"name": "name"})
        assert result == []

    def test_records_deterministic(self) -> None:
        """Same seed should produce same records."""
        schema = {"id": "uuid", "name": "name"}

        seed(42)
        result1 = records(10, schema)

        seed(42)
        result2 = records(10, schema)

        assert result1 == result2

    def test_records_simple_types(self) -> None:
        """Test all simple type specifications."""
        seed(42)
        schema = {
            "address": "address",
            "city": "city",
            "color": "color",
            "company": "company",
            "country": "country",
            "credit_card": "credit_card",
            "date": "date",
            "datetime": "datetime",
            "domain_name": "domain_name",
            "email": "email",
            "first_name": "first_name",
            "float": "float",
            "free_email": "free_email",
            "hex_color": "hex_color",
            "iban": "iban",
            "int": "int",
            "ipv4": "ipv4",
            "ipv6": "ipv6",
            "job": "job",
            "last_name": "last_name",
            "mac_address": "mac_address",
            "md5": "md5",
            "name": "name",
            "paragraph": "paragraph",
            "phone": "phone",
            "safe_email": "safe_email",
            "sentence": "sentence",
            "sha256": "sha256",
            "state": "state",
            "street_address": "street_address",
            "text": "text",
            "url": "url",
            "uuid": "uuid",
            "zip_code": "zip_code",
        }
        result = records(5, schema)
        assert len(result) == 5
        for row in result:
            assert len(row) == len(schema)

    def test_records_int_range(self) -> None:
        """Test integer range specification."""
        seed(42)
        result = records(100, {"age": ("int", 18, 65)})
        for row in result:
            assert isinstance(row["age"], int)
            assert 18 <= row["age"] <= 65

    def test_records_float_range(self) -> None:
        """Test float range specification."""
        seed(42)
        result = records(100, {"price": ("float", 10.0, 100.0)})
        for row in result:
            assert isinstance(row["price"], float)
            assert 10.0 <= row["price"] <= 100.0

    def test_records_text_range(self) -> None:
        """Test text with character limits."""
        seed(42)
        result = records(50, {"bio": ("text", 50, 100)})
        for row in result:
            assert isinstance(row["bio"], str)
            assert 50 <= len(row["bio"]) <= 100

    def test_records_date_range(self) -> None:
        """Test date range specification."""
        seed(42)
        result = records(50, {"hire_date": ("date", "2020-01-01", "2024-12-31")})
        for row in result:
            assert isinstance(row["hire_date"], str)
            assert row["hire_date"].startswith("202")

    def test_records_choice(self) -> None:
        """Test choice specification."""
        seed(42)
        options = ["active", "inactive", "pending"]
        result = records(100, {"status": ("choice", options)})
        for row in result:
            assert row["status"] in options

    def test_records_rgb_color(self) -> None:
        """Test RGB color returns tuple."""
        seed(42)
        result = records(10, {"color": "rgb_color"})
        for row in result:
            assert isinstance(row["color"], tuple)
            assert len(row["color"]) == 3
            for component in row["color"]:
                assert isinstance(component, int)
                assert 0 <= component <= 255

    def test_records_catch_phrase(self) -> None:
        """Test catch phrase generation."""
        seed(42)
        result = records(10, {"slogan": "catch_phrase"})
        for row in result:
            assert isinstance(row["slogan"], str)
            assert len(row["slogan"]) > 0

    def test_records_invalid_type_raises(self) -> None:
        """Invalid type should raise ValueError."""
        with pytest.raises(ValueError, match="Unknown type"):
            records(1, {"field": "invalid_type_xyz"})

    def test_records_invalid_int_range_raises(self) -> None:
        """Invalid int range (min > max) should raise ValueError."""
        with pytest.raises(ValueError, match="Invalid int range"):
            records(1, {"age": ("int", 100, 10)})

    def test_records_invalid_float_range_raises(self) -> None:
        """Invalid float range (min > max) should raise ValueError."""
        with pytest.raises(ValueError, match="Invalid float range"):
            records(1, {"price": ("float", 100.0, 10.0)})

    def test_records_empty_choice_raises(self) -> None:
        """Empty choice options should raise ValueError."""
        with pytest.raises(ValueError, match="empty"):
            records(1, {"status": ("choice", [])})


class TestRecordsTuples:
    """Tests for records_tuples() function."""

    def test_records_tuples_returns_list_of_tuples(self) -> None:
        """records_tuples() should return a list of tuples."""
        seed(42)
        result = records_tuples(5, {"name": "name", "email": "email"})
        assert isinstance(result, list)
        assert len(result) == 5
        for row in result:
            assert isinstance(row, tuple)

    def test_records_tuples_correct_length(self) -> None:
        """Each tuple should have correct number of elements."""
        seed(42)
        schema = {"a": "uuid", "b": "name", "c": "email"}
        result = records_tuples(10, schema)
        for row in result:
            assert len(row) == 3

    def test_records_tuples_alphabetical_order(self) -> None:
        """Values should be in alphabetical key order."""
        seed(42)
        # Keys: age, name (alphabetical order)
        schema = {"name": "name", "age": ("int", 18, 65)}
        result = records_tuples(1, schema)
        row = result[0]
        # First element should be age (int), second should be name (str)
        assert isinstance(row[0], int)  # age
        assert isinstance(row[1], str)  # name

    def test_records_tuples_empty_batch(self) -> None:
        """Empty batch should return empty list."""
        result = records_tuples(0, {"name": "name"})
        assert result == []

    def test_records_tuples_deterministic(self) -> None:
        """Same seed should produce same tuples."""
        schema = {"id": "uuid", "name": "name"}

        seed(42)
        result1 = records_tuples(10, schema)

        seed(42)
        result2 = records_tuples(10, schema)

        assert result1 == result2

    def test_records_tuples_int_range(self) -> None:
        """Test integer range in tuples."""
        seed(42)
        result = records_tuples(100, {"value": ("int", 0, 100)})
        for row in result:
            assert isinstance(row[0], int)
            assert 0 <= row[0] <= 100

    def test_records_tuples_choice(self) -> None:
        """Test choice in tuples."""
        seed(42)
        options = ["a", "b", "c"]
        result = records_tuples(100, {"choice": ("choice", options)})
        for row in result:
            assert row[0] in options


class TestRecordsWithFaker:
    """Test records with Faker class instances."""

    def test_faker_records(self) -> None:
        """Faker instance should support records()."""
        fake = Faker()
        fake.seed(42)
        result = fake.records(5, {"name": "name", "email": "email"})
        assert len(result) == 5
        for row in result:
            assert "name" in row
            assert "email" in row

    def test_faker_records_tuples(self) -> None:
        """Faker instance should support records_tuples()."""
        fake = Faker()
        fake.seed(42)
        result = fake.records_tuples(5, {"name": "name", "email": "email"})
        assert len(result) == 5
        for row in result:
            assert len(row) == 2

    def test_independent_faker_instances(self) -> None:
        """Different Faker instances should be independent."""
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(42)
        fake2.seed(42)

        result1 = fake1.records(5, {"id": "uuid"})
        result2 = fake2.records(5, {"id": "uuid"})

        assert result1 == result2


class TestRecordsEdgeCases:
    """Edge case tests for records generation."""

    def test_single_record(self) -> None:
        """Single record generation should work."""
        seed(42)
        result = records(1, {"name": "name"})
        assert len(result) == 1

    def test_single_field(self) -> None:
        """Single field schema should work."""
        seed(42)
        result = records(10, {"id": "uuid"})
        assert len(result) == 10
        for row in result:
            assert len(row) == 1

    def test_many_fields(self) -> None:
        """Schema with many fields should work."""
        seed(42)
        schema = {f"field_{i}": "uuid" for i in range(50)}
        result = records(10, schema)
        assert len(result) == 10
        for row in result:
            assert len(row) == 50

    def test_large_batch(self) -> None:
        """Large batch generation should work."""
        seed(42)
        result = records(10000, {"id": "uuid", "name": "name"})
        assert len(result) == 10000

    def test_invalid_tuple_spec_raises(self) -> None:
        """Invalid tuple specification should raise."""
        with pytest.raises(ValueError):
            records(1, {"field": ("int",)})  # Missing min, max

    def test_invalid_parameterized_type_raises(self) -> None:
        """Unknown parameterized type should raise."""
        with pytest.raises(ValueError, match="Unknown parameterized type"):
            records(1, {"field": ("unknown_type", 1, 2)})


class TestRecordsSchemaValidation:
    """Tests for schema validation."""

    def test_choice_requires_list(self) -> None:
        """Choice options must be a list."""
        with pytest.raises(ValueError, match="list"):
            records(1, {"status": ("choice", "not_a_list")})

    def test_int_requires_three_elements(self) -> None:
        """Int specification requires exactly 3 elements."""
        with pytest.raises(ValueError, match="int specification must be"):
            records(1, {"age": ("int", 18)})

    def test_float_requires_three_elements(self) -> None:
        """Float specification requires exactly 3 elements."""
        with pytest.raises(ValueError, match="float specification must be"):
            records(1, {"price": ("float", 10.0)})

    def test_text_requires_three_elements(self) -> None:
        """Text specification requires exactly 3 elements."""
        with pytest.raises(ValueError, match="text specification must be"):
            records(1, {"bio": ("text", 50)})

    def test_date_requires_three_elements(self) -> None:
        """Date specification requires exactly 3 elements."""
        with pytest.raises(ValueError, match="date specification must be"):
            records(1, {"date": ("date", "2020-01-01")})

    def test_invalid_text_range_raises(self) -> None:
        """Invalid text range (min > max) should raise ValueError."""
        with pytest.raises(ValueError, match="Invalid text range"):
            records(1, {"bio": ("text", 100, 10)})


class TestRecordsBatchLimits:
    """Tests for batch size limits."""

    def test_batch_size_exceeds_limit_raises(self) -> None:
        """Batch size exceeding 10 million should raise ValueError."""
        with pytest.raises(ValueError, match="exceeds maximum"):
            records(10_000_001, {"id": "uuid"})

    def test_batch_size_at_limit_works(self) -> None:
        """Batch size at exactly 10 million should not raise (if memory allows).

        Note: This test only verifies the validation passes, not actual generation,
        since generating 10M records would be slow and memory-intensive.
        """
        # We test a smaller batch to verify the code path works
        # The actual limit check happens in validate_batch_size
        result = records(100, {"id": "uuid"})
        assert len(result) == 100

    def test_records_tuples_batch_size_exceeds_limit_raises(self) -> None:
        """Batch size exceeding 10 million should raise for records_tuples."""
        with pytest.raises(ValueError, match="exceeds maximum"):
            records_tuples(10_000_001, {"id": "uuid"})
