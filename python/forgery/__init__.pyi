"""Type stubs for the forgery package."""

from collections.abc import Coroutine
from typing import Any

from forgery._forgery import Faker as Faker

__all__: list[str]
__version__: str

# Default Faker instance
fake: Faker

# Module-level convenience functions
def seed(value: int) -> None:
    """Seed the default Faker instance for deterministic output.

    Args:
        value: The seed value.
    """
    ...

def name() -> str:
    """Generate a single random full name.

    Returns:
        A full name (first + last).
    """
    ...

def names(n: int) -> list[str]:
    """Generate a batch of random full names.

    Args:
        n: Number of names to generate.

    Returns:
        A list of full names.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def first_name() -> str:
    """Generate a single random first name.

    Returns:
        A first name.
    """
    ...

def first_names(n: int) -> list[str]:
    """Generate a batch of random first names.

    Args:
        n: Number of first names to generate.

    Returns:
        A list of first names.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def last_name() -> str:
    """Generate a single random last name.

    Returns:
        A last name.
    """
    ...

def last_names(n: int) -> list[str]:
    """Generate a batch of random last names.

    Args:
        n: Number of last names to generate.

    Returns:
        A list of last names.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def email() -> str:
    """Generate a single random email address.

    Returns:
        An email address.
    """
    ...

def emails(n: int) -> list[str]:
    """Generate a batch of random email addresses.

    Args:
        n: Number of emails to generate.

    Returns:
        A list of email addresses.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def integer(min: int = 0, max: int = 100) -> int:
    """Generate a single random integer within a range.

    Args:
        min: Minimum value (inclusive). Default: 0.
        max: Maximum value (inclusive). Default: 100.

    Returns:
        A random integer.

    Raises:
        ValueError: If min > max.
    """
    ...

def integers(n: int, min: int = 0, max: int = 100) -> list[int]:
    """Generate a batch of random integers within a range.

    Args:
        n: Number of integers to generate.
        min: Minimum value (inclusive). Default: 0.
        max: Maximum value (inclusive). Default: 100.

    Returns:
        A list of random integers.

    Raises:
        ValueError: If min > max or n exceeds the maximum batch size (10 million).
    """
    ...

def uuid() -> str:
    """Generate a single random UUID (version 4).

    Returns:
        A UUID string.
    """
    ...

def uuids(n: int) -> list[str]:
    """Generate a batch of random UUIDs (version 4)."""
    ...

# Float generation
def float_(min: float = 0.0, max: float = 1.0) -> float: ...
def floats(n: int, min: float = 0.0, max: float = 1.0) -> list[float]: ...

# Hash generation
def md5() -> str: ...
def md5s(n: int) -> list[str]: ...
def sha256() -> str: ...
def sha256s(n: int) -> list[str]: ...

# Color generation
def color() -> str: ...
def colors(n: int) -> list[str]: ...
def hex_color() -> str: ...
def hex_colors(n: int) -> list[str]: ...
def rgb_color() -> tuple[int, int, int]: ...
def rgb_colors(n: int) -> list[tuple[int, int, int]]: ...

# DateTime generation
def date(start: str = "2000-01-01", end: str = "2030-12-31") -> str: ...
def dates(n: int, start: str = "2000-01-01", end: str = "2030-12-31") -> list[str]: ...
def date_of_birth(min_age: int = 18, max_age: int = 80) -> str: ...
def dates_of_birth(n: int, min_age: int = 18, max_age: int = 80) -> list[str]: ...
def datetime_(start: str = "2000-01-01", end: str = "2030-12-31") -> str: ...
def datetimes(n: int, start: str = "2000-01-01", end: str = "2030-12-31") -> list[str]: ...

# Text generation
def sentence(word_count: int = 10) -> str: ...
def sentences(n: int, word_count: int = 10) -> list[str]: ...
def paragraph(sentence_count: int = 5) -> str: ...
def paragraphs(n: int, sentence_count: int = 5) -> list[str]: ...
def text(min_chars: int = 50, max_chars: int = 200) -> str: ...
def texts(n: int, min_chars: int = 50, max_chars: int = 200) -> list[str]: ...

# Address generation
def street_address() -> str: ...
def street_addresses(n: int) -> list[str]: ...
def city() -> str: ...
def cities(n: int) -> list[str]: ...
def state() -> str: ...
def states(n: int) -> list[str]: ...
def country() -> str: ...
def countries(n: int) -> list[str]: ...
def zip_code() -> str: ...
def zip_codes(n: int) -> list[str]: ...
def address() -> str: ...
def addresses(n: int) -> list[str]: ...

# Phone generation
def phone_number() -> str: ...
def phone_numbers(n: int) -> list[str]: ...

# Company generation
def company() -> str: ...
def companies(n: int) -> list[str]: ...
def job() -> str: ...
def jobs(n: int) -> list[str]: ...
def catch_phrase() -> str: ...
def catch_phrases(n: int) -> list[str]: ...

# Network generation
def url() -> str: ...
def urls(n: int) -> list[str]: ...
def domain_name() -> str: ...
def domain_names(n: int) -> list[str]: ...
def ipv4() -> str: ...
def ipv4s(n: int) -> list[str]: ...
def ipv6() -> str: ...
def ipv6s(n: int) -> list[str]: ...
def mac_address() -> str: ...
def mac_addresses(n: int) -> list[str]: ...

# Email variants
def safe_email() -> str: ...
def safe_emails(n: int) -> list[str]: ...
def free_email() -> str: ...
def free_emails(n: int) -> list[str]: ...

# Finance generation
def credit_card() -> str: ...
def credit_cards(n: int) -> list[str]: ...
def iban() -> str: ...
def ibans(n: int) -> list[str]: ...
def bic() -> str: ...
def bics(n: int) -> list[str]: ...
def bank_account() -> str: ...
def bank_accounts(n: int) -> list[str]: ...
def bank_name() -> str: ...
def bank_names(n: int) -> list[str]: ...

# UK Banking generation
def sort_code() -> str:
    """Generate a single UK sort code (format: XX-XX-XX)."""
    ...

def sort_codes(n: int) -> list[str]:
    """Generate a batch of UK sort codes.

    Args:
        n: Number of sort codes to generate.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def uk_account_number() -> str:
    """Generate a single UK bank account number (8 digits)."""
    ...

def uk_account_numbers(n: int) -> list[str]:
    """Generate a batch of UK bank account numbers (8 digits each).

    Args:
        n: Number of account numbers to generate.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

# Transaction generation
def transactions(
    n: int,
    starting_balance: float,
    start_date: str,
    end_date: str,
) -> list[dict[str, str | float]]:
    """Generate a batch of financial transactions.

    Each transaction is a dictionary with keys:
    - reference: 8-character alphanumeric reference
    - date: Transaction date in YYYY-MM-DD format
    - amount: Transaction amount (negative for debits)
    - transaction_type: e.g., "Direct Debit", "Card Payment", etc.
    - description: Transaction description
    - balance: Running balance after transaction

    Args:
        n: Number of transactions to generate.
        starting_balance: Opening balance before first transaction.
        start_date: Start date in YYYY-MM-DD format.
        end_date: End date in YYYY-MM-DD format.

    Returns:
        List of transaction dictionaries, sorted chronologically.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

def transaction_amount(min: float, max: float) -> float:
    """Generate a single transaction amount.

    Args:
        min: Minimum amount (inclusive).
        max: Maximum amount (inclusive).

    Returns:
        A transaction amount rounded to 2 decimal places.
    """
    ...

def transaction_amounts(n: int, min: float, max: float) -> list[float]:
    """Generate a batch of transaction amounts.

    Args:
        n: Number of amounts to generate.
        min: Minimum amount (inclusive).
        max: Maximum amount (inclusive).

    Returns:
        List of amounts rounded to 2 decimal places.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million).
    """
    ...

# Password generation
def password(
    length: int = 12,
    uppercase: bool = True,
    lowercase: bool = True,
    digits: bool = True,
    symbols: bool = True,
) -> str:
    """Generate a single random password.

    Args:
        length: Length of the password (default: 12).
        uppercase: Include uppercase letters (default: True).
        lowercase: Include lowercase letters (default: True).
        digits: Include digits (default: True).
        symbols: Include symbols (default: True).

    Raises:
        ValueError: If no character sets are enabled.
    """
    ...

def passwords(
    n: int,
    length: int = 12,
    uppercase: bool = True,
    lowercase: bool = True,
    digits: bool = True,
    symbols: bool = True,
) -> list[str]:
    """Generate a batch of random passwords.

    Args:
        n: Number of passwords to generate.
        length: Length of each password (default: 12).
        uppercase: Include uppercase letters (default: True).
        lowercase: Include lowercase letters (default: True).
        digits: Include digits (default: True).
        symbols: Include symbols (default: True).

    Raises:
        ValueError: If no character sets are enabled or n exceeds batch limit.
    """
    ...

# Records generation
FieldValue = str | int | float | tuple[int, int, int]
SimpleType = str
IntRangeSpec = tuple[str, int, int]
FloatRangeSpec = tuple[str, float, float]
TextSpec = tuple[str, int, int]
DateRangeSpec = tuple[str, str, str]
ChoiceSpec = tuple[str, list[str]]
FieldSpec = SimpleType | IntRangeSpec | FloatRangeSpec | TextSpec | DateRangeSpec | ChoiceSpec
Schema = dict[str, FieldSpec]

def records(n: int, schema: Schema) -> list[dict[str, FieldValue]]:
    """Generate structured records based on a schema.

    Args:
        n: Number of records to generate.
        schema: Dictionary mapping field names to type specifications.
            - Simple types: "name", "email", "uuid", "int", "float", etc.
            - Integer range: ("int", min, max)
            - Float range: ("float", min, max)
            - Text with limits: ("text", min_chars, max_chars)
            - Date range: ("date", start, end)
            - Choice: ("choice", ["option1", "option2", ...])

    Returns:
        A list of dictionaries, each representing a record.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million)
            or if the schema contains invalid specifications.
    """
    ...

def records_tuples(n: int, schema: Schema) -> list[tuple[FieldValue, ...]]:
    """Generate structured records as tuples based on a schema.

    This is faster than records() since it avoids creating dictionaries.
    Values are returned in alphabetical order of field names.

    Args:
        n: Number of records to generate.
        schema: Dictionary mapping field names to type specifications.

    Returns:
        A list of tuples, each representing a record with values in
        alphabetical order of field names.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million)
            or if the schema contains invalid specifications.
    """
    ...

def records_arrow(n: int, schema: Schema) -> Any:
    """Generate structured records as a PyArrow RecordBatch.

    This is the high-performance path for generating structured data,
    suitable for use with PyArrow, Polars, and other Arrow-compatible tools.

    The data is generated in columnar format and returned as a PyArrow
    RecordBatch, which can be converted to pandas DataFrames, Polars
    DataFrames, or used directly with Arrow-based processing tools.

    Note:
        Requires pyarrow to be installed: pip install pyarrow

        Columns are ordered alphabetically by field name (not by Python dict
        insertion order), matching the behavior of records_tuples().

    Args:
        n: Number of records to generate.
        schema: Dictionary mapping field names to type specifications.

    Returns:
        A pyarrow.RecordBatch with the generated data.

    Raises:
        ValueError: If n exceeds the maximum batch size (10 million)
            or if the schema contains invalid specifications.
        ImportError: If pyarrow is not installed.
    """
    ...

# Async Records generation

def records_async(
    n: int, schema: Schema, chunk_size: int | None = None
) -> Coroutine[Any, Any, list[dict[str, FieldValue]]]:
    """Generate structured records asynchronously for non-blocking batch generation.

    Args:
        n: Number of records to generate.
        schema: Dictionary mapping field names to type specifications.
        chunk_size: Number of records per chunk (default: 10,000).

    Returns:
        A coroutine that resolves to a list of dictionaries.
    """
    ...

def records_tuples_async(
    n: int, schema: Schema, chunk_size: int | None = None
) -> Coroutine[Any, Any, list[tuple[FieldValue, ...]]]:
    """Generate structured records as tuples asynchronously.

    Args:
        n: Number of records to generate.
        schema: Dictionary mapping field names to type specifications.
        chunk_size: Number of records per chunk (default: 10,000).

    Returns:
        A coroutine that resolves to a list of tuples.
    """
    ...

def records_arrow_async(
    n: int, schema: Schema, chunk_size: int | None = None
) -> Coroutine[Any, Any, Any]:
    """Generate structured records as a PyArrow RecordBatch asynchronously.

    Args:
        n: Number of records to generate.
        schema: Dictionary mapping field names to type specifications.
        chunk_size: Number of records per chunk (default: 10,000).

    Returns:
        A coroutine that resolves to a pyarrow.RecordBatch.

    Note:
        Requires pyarrow to be installed.
    """
    ...

# Custom Providers
def add_provider(name: str, options: list[str]) -> None:
    """Register a custom provider on the default Faker instance.

    Each option has equal probability of being selected.

    Args:
        name: The provider name (must not conflict with built-in types).
        options: List of string options to choose from.

    Raises:
        ValueError: If name conflicts with built-in type or options is empty.
    """
    ...

def add_weighted_provider(name: str, weighted_options: list[tuple[str, int]]) -> None:
    """Register a weighted custom provider on the default Faker instance.

    Options are selected based on their relative weights. Higher weights mean
    the option is more likely to be selected.

    Args:
        name: The provider name (must not conflict with built-in types).
        weighted_options: List of (value, weight) tuples.

    Raises:
        ValueError: If name conflicts, options empty, or weights invalid.
    """
    ...

def remove_provider(name: str) -> bool:
    """Remove a custom provider from the default Faker instance.

    Args:
        name: The provider name to remove.

    Returns:
        True if provider was removed, False if it didn't exist.
    """
    ...

def has_provider(name: str) -> bool:
    """Check if a custom provider exists on the default Faker instance.

    Args:
        name: The provider name to check.

    Returns:
        True if provider exists, False otherwise.
    """
    ...

def list_providers() -> list[str]:
    """List all custom provider names on the default Faker instance.

    Returns:
        List of registered custom provider names.
    """
    ...

def generate(name: str) -> str:
    """Generate a single value from a custom provider.

    Args:
        name: The custom provider name.

    Returns:
        A randomly selected string from the provider's options.

    Raises:
        ValueError: If provider doesn't exist.
    """
    ...

def generate_batch(name: str, n: int) -> list[str]:
    """Generate a batch of values from a custom provider.

    Args:
        name: The custom provider name.
        n: Number of values to generate.

    Returns:
        A list of randomly selected strings.

    Raises:
        ValueError: If provider doesn't exist or n exceeds batch limit.
    """
    ...
