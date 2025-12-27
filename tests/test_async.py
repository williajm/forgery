"""Tests for async record generation."""

import asyncio

import pyarrow as pa
import pytest

from forgery import (
    Faker,
    records_arrow_async,
    records_async,
    records_tuples_async,
    seed,
)


class TestRecordsAsync:
    """Tests for records_async() function."""

    @pytest.mark.asyncio
    async def test_basic_generation(self) -> None:
        """Test basic async record generation."""
        fake = Faker()
        fake.seed(42)

        records = await fake.records_async(100, {"name": "name", "email": "email"})

        assert len(records) == 100
        for record in records:
            assert "name" in record
            assert "email" in record
            assert isinstance(record["name"], str)
            assert isinstance(record["email"], str)

    @pytest.mark.asyncio
    async def test_determinism(self) -> None:
        """Test that async generation is deterministic with same seed."""
        fake1 = Faker()
        fake1.seed(42)
        fake2 = Faker()
        fake2.seed(42)

        records1 = await fake1.records_async(100, {"name": "name"})
        records2 = await fake2.records_async(100, {"name": "name"})

        assert records1 == records2

    @pytest.mark.asyncio
    async def test_same_as_sync(self) -> None:
        """Test that async produces same results as sync with same seed."""
        fake1 = Faker()
        fake1.seed(42)
        fake2 = Faker()
        fake2.seed(42)

        sync_records = fake1.records(100, {"name": "name", "age": ("int", 18, 65)})
        async_records = await fake2.records_async(100, {"name": "name", "age": ("int", 18, 65)})

        assert sync_records == async_records

    @pytest.mark.asyncio
    async def test_empty_generation(self) -> None:
        """Test generating zero records."""
        fake = Faker()

        records = await fake.records_async(0, {"name": "name"})

        assert records == []

    @pytest.mark.asyncio
    async def test_custom_chunk_size(self) -> None:
        """Test generation with custom chunk size."""
        fake = Faker()
        fake.seed(42)

        # Small chunk size to test chunking behavior
        records = await fake.records_async(100, {"name": "name"}, chunk_size=10)

        assert len(records) == 100

    @pytest.mark.asyncio
    async def test_module_level_function(self) -> None:
        """Test module-level records_async function."""
        seed(42)

        records = await records_async(50, {"name": "name", "uuid": "uuid"})

        assert len(records) == 50
        for record in records:
            assert "name" in record
            assert "uuid" in record

    @pytest.mark.asyncio
    async def test_allows_concurrent_tasks(self) -> None:
        """Test that async generation allows other tasks to run."""
        fake = Faker()
        fake.seed(42)
        other_task_ran = False

        async def other_task() -> None:
            nonlocal other_task_ran
            await asyncio.sleep(0)  # Yield control
            other_task_ran = True

        # Run generation and other task concurrently
        # Using small chunk size to ensure yielding
        results, _ = await asyncio.gather(
            fake.records_async(1000, {"name": "name"}, chunk_size=100),
            other_task(),
        )

        assert other_task_ran
        assert len(results) == 1000

    @pytest.mark.asyncio
    async def test_with_all_field_types(self) -> None:
        """Test async generation with various field types."""
        fake = Faker()
        fake.seed(42)

        schema = {
            "name": "name",
            "email": "email",
            "age": ("int", 18, 65),
            "salary": ("float", 30000.0, 150000.0),
            "status": ("choice", ["active", "inactive"]),
            "bio": ("text", 10, 50),
        }

        records = await fake.records_async(10, schema)

        assert len(records) == 10
        for record in records:
            assert isinstance(record["name"], str)
            assert isinstance(record["email"], str)
            assert isinstance(record["age"], int)
            assert 18 <= record["age"] <= 65
            assert isinstance(record["salary"], float)
            assert record["status"] in ["active", "inactive"]


class TestRecordsTuplesAsync:
    """Tests for records_tuples_async() function."""

    @pytest.mark.asyncio
    async def test_basic_generation(self) -> None:
        """Test basic async tuple generation."""
        fake = Faker()
        fake.seed(42)

        records = await fake.records_tuples_async(100, {"age": ("int", 18, 65), "name": "name"})

        assert len(records) == 100
        for record in records:
            # Fields in alphabetical order: age, name
            assert len(record) == 2
            assert isinstance(record[0], int)  # age
            assert isinstance(record[1], str)  # name

    @pytest.mark.asyncio
    async def test_determinism(self) -> None:
        """Test that async tuple generation is deterministic."""
        fake1 = Faker()
        fake1.seed(42)
        fake2 = Faker()
        fake2.seed(42)

        records1 = await fake1.records_tuples_async(100, {"name": "name"})
        records2 = await fake2.records_tuples_async(100, {"name": "name"})

        assert records1 == records2

    @pytest.mark.asyncio
    async def test_same_as_sync(self) -> None:
        """Test that async produces same results as sync."""
        fake1 = Faker()
        fake1.seed(42)
        fake2 = Faker()
        fake2.seed(42)

        sync_records = fake1.records_tuples(100, {"name": "name"})
        async_records = await fake2.records_tuples_async(100, {"name": "name"})

        assert sync_records == async_records

    @pytest.mark.asyncio
    async def test_module_level_function(self) -> None:
        """Test module-level records_tuples_async function."""
        seed(42)

        records = await records_tuples_async(50, {"name": "name"})

        assert len(records) == 50


class TestRecordsArrowAsync:
    """Tests for records_arrow_async() function."""

    @pytest.mark.asyncio
    async def test_basic_generation(self) -> None:
        """Test basic async Arrow generation."""
        fake = Faker()
        fake.seed(42)

        batch = await fake.records_arrow_async(100, {"name": "name", "age": ("int", 18, 65)})

        assert isinstance(batch, pa.RecordBatch)
        assert batch.num_rows == 100
        assert batch.num_columns == 2

    @pytest.mark.asyncio
    async def test_determinism(self) -> None:
        """Test that async Arrow generation is deterministic."""
        fake1 = Faker()
        fake1.seed(42)
        fake2 = Faker()
        fake2.seed(42)

        batch1 = await fake1.records_arrow_async(100, {"name": "name"})
        batch2 = await fake2.records_arrow_async(100, {"name": "name"})

        assert batch1.equals(batch2)

    @pytest.mark.asyncio
    async def test_same_as_sync(self) -> None:
        """Test that async produces same results as sync."""
        fake1 = Faker()
        fake1.seed(42)
        fake2 = Faker()
        fake2.seed(42)

        sync_batch = fake1.records_arrow(100, {"name": "name"})
        async_batch = await fake2.records_arrow_async(100, {"name": "name"})

        assert sync_batch.equals(async_batch)

    @pytest.mark.asyncio
    async def test_chunked_generation(self) -> None:
        """Test Arrow generation with chunking."""
        fake = Faker()
        fake.seed(42)

        # Generate more records than chunk size
        batch = await fake.records_arrow_async(1000, {"name": "name"}, chunk_size=100)

        assert batch.num_rows == 1000

    @pytest.mark.asyncio
    async def test_module_level_function(self) -> None:
        """Test module-level records_arrow_async function."""
        seed(42)

        batch = await records_arrow_async(50, {"name": "name", "uuid": "uuid"})

        assert isinstance(batch, pa.RecordBatch)
        assert batch.num_rows == 50
        assert batch.num_columns == 2

    @pytest.mark.asyncio
    async def test_column_types(self) -> None:
        """Test that Arrow columns have correct types."""
        fake = Faker()
        fake.seed(42)

        schema = {
            "age": ("int", 18, 65),
            "name": "name",
            "salary": ("float", 30000.0, 150000.0),
        }

        batch = await fake.records_arrow_async(10, schema)

        # Columns are in alphabetical order
        assert batch.schema.field(0).name == "age"
        assert batch.schema.field(0).type == pa.int64()
        assert batch.schema.field(1).name == "name"
        assert batch.schema.field(1).type == pa.utf8()
        assert batch.schema.field(2).name == "salary"
        assert batch.schema.field(2).type == pa.float64()


class TestAsyncWithCustomProviders:
    """Tests for async generation with custom providers."""

    @pytest.mark.asyncio
    async def test_records_async_with_custom_provider(self) -> None:
        """Test async records with custom provider."""
        fake = Faker()
        fake.seed(42)
        fake.add_provider("department", ["Engineering", "Sales", "HR"])

        records = await fake.records_async(100, {"name": "name", "dept": "department"})

        assert len(records) == 100
        for record in records:
            assert record["dept"] in ["Engineering", "Sales", "HR"]

    @pytest.mark.asyncio
    async def test_records_arrow_async_with_custom_provider(self) -> None:
        """Test async Arrow with custom provider."""
        fake = Faker()
        fake.seed(42)
        fake.add_provider("status", ["active", "inactive"])

        batch = await fake.records_arrow_async(100, {"name": "name", "status": "status"})

        assert batch.num_rows == 100
        # Verify custom provider values
        status_array = batch.column("status")
        for i in range(batch.num_rows):
            assert status_array[i].as_py() in ["active", "inactive"]


class TestAsyncErrorHandling:
    """Tests for async error handling."""

    @pytest.mark.asyncio
    async def test_invalid_schema(self) -> None:
        """Test that invalid schema raises error."""
        fake = Faker()

        with pytest.raises(ValueError):
            await fake.records_async(10, {"name": "invalid_type"})

    @pytest.mark.asyncio
    async def test_batch_size_limit(self) -> None:
        """Test that batch size limit is enforced."""
        fake = Faker()

        with pytest.raises(ValueError, match="exceeds maximum"):
            await fake.records_async(
                100_000_000,  # 100M exceeds 10M limit
                {"name": "name"},
            )
