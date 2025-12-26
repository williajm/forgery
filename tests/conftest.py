"""Pytest configuration and fixtures."""

import pytest

from forgery import Faker, seed


@pytest.fixture(autouse=True)
def reset_seed() -> None:
    """Reset the module-level seed before each test."""
    seed(0)


@pytest.fixture
def faker() -> Faker:
    """Provide a seeded Faker instance."""
    fake = Faker()
    fake.seed(42)
    return fake
