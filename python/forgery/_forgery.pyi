"""Type stubs for the Rust extension module."""

import builtins
from collections.abc import Coroutine
from typing import Any

# Records schema types (matching forgery/__init__.pyi for consistency)
FieldValue = str | int | float | tuple[int, int, int]
SimpleType = str
IntRangeSpec = tuple[str, int, int]
FloatRangeSpec = tuple[str, float, float]
TextSpec = tuple[str, int, int]
DateRangeSpec = tuple[str, str, str]
ChoiceSpec = tuple[str, list[str]]
FieldSpec = SimpleType | IntRangeSpec | FloatRangeSpec | TextSpec | DateRangeSpec | ChoiceSpec
Schema = dict[str, FieldSpec]

class Faker:
    """A fake data generator with its own random state.

    Each instance maintains independent RNG state, allowing for deterministic
    generation when seeded. The default locale is "en_US".

    Supported Locales:
        - en_US: English (United States) - default
        - en_GB: English (United Kingdom)
        - de_DE: German (Germany)
        - fr_FR: French (France)
        - es_ES: Spanish (Spain)
        - it_IT: Italian (Italy)
        - ja_JP: Japanese (Japan)

    Example:
        >>> from forgery import Faker
        >>> fake = Faker()
        >>> fake.seed(42)
        >>> names = fake.names(100)

        >>> german_fake = Faker("de_DE")
        >>> german_fake.names(10)  # German names
    """

    def __init__(self, locale: str = "en_US") -> None:
        """Create a new Faker instance with the specified locale.

        Args:
            locale: The locale for generated data (default: "en_US").
                    Supported: en_US, en_GB, de_DE, fr_FR, es_ES, it_IT, ja_JP.

        Raises:
            ValueError: If locale is not supported.
        """
        ...

    def seed(self, value: int) -> None:
        """Seed the random number generator for deterministic output.

        Args:
            value: The seed value.
        """
        ...

    # Name generators
    def name(self) -> str:
        """Generate a single random full name."""
        ...

    def names(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random full names.

        Args:
            n: Number of names to generate.
            unique: If True, ensure all generated values are unique.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million) or
                        unique generation cannot produce enough unique values.
        """
        ...

    def first_name(self) -> str:
        """Generate a single random first name."""
        ...

    def first_names(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random first names.

        Args:
            n: Number of first names to generate.
            unique: If True, ensure all generated values are unique.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million) or
                        unique generation cannot produce enough unique values.
        """
        ...

    def last_name(self) -> str:
        """Generate a single random last name."""
        ...

    def last_names(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random last names.

        Args:
            n: Number of last names to generate.
            unique: If True, ensure all generated values are unique.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million) or
                        unique generation cannot produce enough unique values.
        """
        ...

    # Internet generators
    def email(self) -> str:
        """Generate a single random email address."""
        ...

    def emails(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random email addresses.

        Args:
            n: Number of emails to generate.
            unique: If True, ensure all generated values are unique.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million) or
                        unique generation cannot produce enough unique values.
        """
        ...

    # Number generators
    def integer(self, min: int = 0, max: int = 100) -> int:
        """Generate a single random integer within a range.

        Args:
            min: Minimum value (inclusive).
            max: Maximum value (inclusive).

        Raises:
            ValueError: If min > max.
        """
        ...

    def integers(self, n: int, min: int = 0, max: int = 100) -> list[int]:
        """Generate a batch of random integers within a range.

        Args:
            n: Number of integers to generate.
            min: Minimum value (inclusive).
            max: Maximum value (inclusive).

        Raises:
            ValueError: If min > max or n exceeds the maximum batch size (10 million).
        """
        ...

    # Identifier generators
    def uuid(self) -> str:
        """Generate a single random UUID (version 4)."""
        ...

    def uuids(self, n: int) -> list[str]:
        """Generate a batch of random UUIDs (version 4).

        Args:
            n: Number of UUIDs to generate.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million).
        """
        ...

    def md5(self) -> str:
        """Generate a single random MD5 hash."""
        ...

    def md5s(self, n: int) -> list[str]:
        """Generate a batch of random MD5 hashes."""
        ...

    def sha256(self) -> str:
        """Generate a single random SHA256 hash."""
        ...

    def sha256s(self, n: int) -> list[str]:
        """Generate a batch of random SHA256 hashes."""
        ...

    # Float generators
    def float(self, min: builtins.float = 0.0, max: builtins.float = 1.0) -> builtins.float:
        """Generate a single random float within a range."""
        ...

    def floats(
        self, n: int, min: builtins.float = 0.0, max: builtins.float = 1.0
    ) -> list[builtins.float]:
        """Generate a batch of random floats within a range."""
        ...

    # Color generators
    def color(self) -> str:
        """Generate a single random color name."""
        ...

    def colors(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random color names.

        Args:
            n: Number of colors to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def hex_color(self) -> str:
        """Generate a single random hex color (#RRGGBB)."""
        ...

    def hex_colors(self, n: int) -> list[str]:
        """Generate a batch of random hex colors."""
        ...

    def rgb_color(self) -> tuple[int, int, int]:
        """Generate a single random RGB color tuple."""
        ...

    def rgb_colors(self, n: int) -> list[tuple[int, int, int]]:
        """Generate a batch of random RGB color tuples."""
        ...

    # DateTime generators
    def date(self, start: str = "2000-01-01", end: str = "2030-12-31") -> str:
        """Generate a single random date (YYYY-MM-DD format)."""
        ...

    def dates(self, n: int, start: str = "2000-01-01", end: str = "2030-12-31") -> list[str]:
        """Generate a batch of random dates."""
        ...

    def date_of_birth(self, min_age: int = 18, max_age: int = 80) -> str:
        """Generate a single random date of birth."""
        ...

    def dates_of_birth(self, n: int, min_age: int = 18, max_age: int = 80) -> list[str]:
        """Generate a batch of random dates of birth."""
        ...

    def datetime(self, start: str = "2000-01-01", end: str = "2030-12-31") -> str:
        """Generate a single random datetime (ISO 8601 format)."""
        ...

    def datetimes(self, n: int, start: str = "2000-01-01", end: str = "2030-12-31") -> list[str]:
        """Generate a batch of random datetimes."""
        ...

    # Text generators
    def sentence(self, word_count: int = 10) -> str:
        """Generate a single random sentence."""
        ...

    def sentences(self, n: int, word_count: int = 10) -> list[str]:
        """Generate a batch of random sentences."""
        ...

    def paragraph(self, sentence_count: int = 5) -> str:
        """Generate a single random paragraph."""
        ...

    def paragraphs(self, n: int, sentence_count: int = 5) -> list[str]:
        """Generate a batch of random paragraphs."""
        ...

    def text(self, min_chars: int = 50, max_chars: int = 200) -> str:
        """Generate a single random text block."""
        ...

    def texts(self, n: int, min_chars: int = 50, max_chars: int = 200) -> list[str]:
        """Generate a batch of random text blocks."""
        ...

    # Address generators
    def street_address(self) -> str:
        """Generate a single random street address."""
        ...

    def street_addresses(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random street addresses.

        Args:
            n: Number of addresses to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def city(self) -> str:
        """Generate a single random city name."""
        ...

    def cities(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random city names.

        Args:
            n: Number of cities to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def state(self) -> str:
        """Generate a single random state name."""
        ...

    def states(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random state names.

        Args:
            n: Number of states to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def country(self) -> str:
        """Generate a single random country name."""
        ...

    def countries(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random country names.

        Args:
            n: Number of countries to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def zip_code(self) -> str:
        """Generate a single random zip code."""
        ...

    def zip_codes(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random zip codes.

        Args:
            n: Number of zip codes to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def address(self) -> str:
        """Generate a single random full address."""
        ...

    def addresses(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random full addresses.

        Args:
            n: Number of addresses to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    # Phone generators
    def phone_number(self) -> str:
        """Generate a single random phone number."""
        ...

    def phone_numbers(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random phone numbers.

        Args:
            n: Number of phone numbers to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    # Company generators
    def company(self) -> str:
        """Generate a single random company name."""
        ...

    def companies(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random company names.

        Args:
            n: Number of companies to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def job(self) -> str:
        """Generate a single random job title."""
        ...

    def jobs(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random job titles.

        Args:
            n: Number of job titles to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def catch_phrase(self) -> str:
        """Generate a single random catch phrase."""
        ...

    def catch_phrases(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random catch phrases.

        Args:
            n: Number of catch phrases to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    # Network generators
    def url(self) -> str:
        """Generate a single random URL."""
        ...

    def urls(self, n: int) -> list[str]:
        """Generate a batch of random URLs."""
        ...

    def domain_name(self) -> str:
        """Generate a single random domain name."""
        ...

    def domain_names(self, n: int) -> list[str]:
        """Generate a batch of random domain names."""
        ...

    def ipv4(self) -> str:
        """Generate a single random IPv4 address."""
        ...

    def ipv4s(self, n: int) -> list[str]:
        """Generate a batch of random IPv4 addresses."""
        ...

    def ipv6(self) -> str:
        """Generate a single random IPv6 address."""
        ...

    def ipv6s(self, n: int) -> list[str]:
        """Generate a batch of random IPv6 addresses."""
        ...

    def mac_address(self) -> str:
        """Generate a single random MAC address."""
        ...

    def mac_addresses(self, n: int) -> list[str]:
        """Generate a batch of random MAC addresses."""
        ...

    # Email variants
    def safe_email(self) -> str:
        """Generate a single random safe email (example.com/org/net)."""
        ...

    def safe_emails(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random safe emails.

        Args:
            n: Number of emails to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    def free_email(self) -> str:
        """Generate a single random free email (gmail.com, etc.)."""
        ...

    def free_emails(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random free emails.

        Args:
            n: Number of emails to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    # Finance generators
    def credit_card(self) -> str:
        """Generate a single random credit card number with valid Luhn checksum."""
        ...

    def credit_cards(self, n: int) -> list[str]:
        """Generate a batch of random credit card numbers."""
        ...

    def iban(self) -> str:
        """Generate a single random IBAN with valid checksum."""
        ...

    def ibans(self, n: int) -> list[str]:
        """Generate a batch of random IBANs."""
        ...

    def bic(self) -> str:
        """Generate a single random BIC/SWIFT code."""
        ...

    def bics(self, n: int) -> list[str]:
        """Generate a batch of random BIC/SWIFT codes."""
        ...

    def bank_account(self) -> str:
        """Generate a single random bank account number (8-17 digits)."""
        ...

    def bank_accounts(self, n: int) -> list[str]:
        """Generate a batch of random bank account numbers."""
        ...

    def bank_name(self) -> str:
        """Generate a single random bank name (locale-specific)."""
        ...

    def bank_names(self, n: int, unique: bool = False) -> list[str]:
        """Generate a batch of random bank names.

        Args:
            n: Number of bank names to generate.
            unique: If True, ensure all generated values are unique.
        """
        ...

    # UK Banking generators
    def sort_code(self) -> str:
        """Generate a single UK sort code (format: XX-XX-XX)."""
        ...

    def sort_codes(self, n: int) -> list[str]:
        """Generate a batch of UK sort codes.

        Args:
            n: Number of sort codes to generate.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million).
        """
        ...

    def uk_account_number(self) -> str:
        """Generate a single UK bank account number (8 digits)."""
        ...

    def uk_account_numbers(self, n: int) -> list[str]:
        """Generate a batch of UK bank account numbers (8 digits each).

        Args:
            n: Number of account numbers to generate.

        Raises:
            ValueError: If n exceeds the maximum batch size (10 million).
        """
        ...

    # Transaction generators
    def transactions(
        self,
        n: int,
        starting_balance: builtins.float,
        start_date: str,
        end_date: str,
    ) -> list[dict[str, str | builtins.float]]:
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

    def transaction_amount(self, min: builtins.float, max: builtins.float) -> builtins.float:
        """Generate a single transaction amount.

        Args:
            min: Minimum amount (inclusive).
            max: Maximum amount (inclusive).

        Returns:
            A transaction amount rounded to 2 decimal places.
        """
        ...

    def transaction_amounts(
        self, n: int, min: builtins.float, max: builtins.float
    ) -> list[builtins.float]:
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

    # Password generators
    def password(
        self,
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
        self,
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

    # Records generators
    def records(self, n: int, schema: Schema) -> list[dict[str, FieldValue]]:
        """Generate structured records based on a schema.

        The schema is a dictionary mapping field names to type specifications:
        - Simple types: "name", "email", "uuid", "int", "float", etc.
        - Integer range: ("int", min, max)
        - Float range: ("float", min, max)
        - Text with limits: ("text", min_chars, max_chars)
        - Date range: ("date", start, end)
        - Choice: ("choice", ["option1", "option2", ...])

        Args:
            n: Number of records to generate.
            schema: Dictionary mapping field names to type specifications.

        Returns:
            A list of dictionaries, each containing the generated fields.

        Raises:
            ValueError: If n exceeds the maximum batch size or schema is invalid.
        """
        ...

    def records_tuples(self, n: int, schema: Schema) -> list[tuple[FieldValue, ...]]:
        """Generate structured records as tuples based on a schema.

        This is faster than records() since it avoids creating dictionaries.
        Values are returned in alphabetical order of the schema keys.

        Args:
            n: Number of records to generate.
            schema: Dictionary mapping field names to type specifications.

        Returns:
            A list of tuples, each containing values in alphabetical key order.

        Raises:
            ValueError: If n exceeds the maximum batch size or schema is invalid.
        """
        ...

    def records_arrow(self, n: int, schema: Schema) -> Any:
        """Generate structured records as a PyArrow RecordBatch.

        This is the high-performance path for generating structured data,
        suitable for use with PyArrow, Polars, and other Arrow-compatible tools.

        The data is generated in columnar format and returned as a PyArrow
        RecordBatch, which can be converted to pandas DataFrames, Polars
        DataFrames, or used directly with Arrow-based processing tools.

        Note:
            Requires pyarrow to be installed: pip install pyarrow

        Args:
            n: Number of records to generate.
            schema: Dictionary mapping field names to type specifications.

        Returns:
            A pyarrow.RecordBatch with the generated data.

        Raises:
            ValueError: If n exceeds the maximum batch size or schema is invalid.
            ImportError: If pyarrow is not installed.
        """
        ...

    # Async records generators
    def records_async(
        self, n: int, schema: Schema, chunk_size: int | None = None
    ) -> Coroutine[Any, Any, list[dict[str, FieldValue]]]:
        """Generate structured records asynchronously for non-blocking batch generation.

        This method generates records in chunks, yielding control between chunks
        to allow other async tasks to run. Ideal for generating millions of records
        without blocking the event loop.

        Note on RNG State:
            The async methods use a snapshot of the RNG state at call time. The main
            Faker instance's RNG is not advanced. For different results on each call,
            create separate Faker instances or re-seed between calls.

        Args:
            n: Number of records to generate.
            schema: Dictionary mapping field names to type specifications.
            chunk_size: Number of records per chunk (default: 10,000).

        Returns:
            A coroutine that resolves to a list of dictionaries.

        Raises:
            ValueError: If n exceeds the maximum batch size or schema is invalid.
        """
        ...

    def records_tuples_async(
        self, n: int, schema: Schema, chunk_size: int | None = None
    ) -> Coroutine[Any, Any, list[tuple[FieldValue, ...]]]:
        """Generate structured records as tuples asynchronously.

        Similar to records_async() but returns tuples instead of dictionaries,
        which is faster for large datasets. Values are returned in alphabetical
        order of the schema keys.

        Args:
            n: Number of records to generate.
            schema: Dictionary mapping field names to type specifications.
            chunk_size: Number of records per chunk (default: 10,000).

        Returns:
            A coroutine that resolves to a list of tuples.

        Raises:
            ValueError: If n exceeds the maximum batch size or schema is invalid.
        """
        ...

    def records_arrow_async(
        self, n: int, schema: Schema, chunk_size: int | None = None
    ) -> Coroutine[Any, Any, Any]:
        """Generate structured records as a PyArrow RecordBatch asynchronously.

        This is the high-performance async path for generating structured data.
        Generates data in chunks and concatenates them into a single RecordBatch.

        Important: Chunking Affects Output
            When n > chunk_size, the output differs from records_arrow() due to
            column-major RNG consumption within each chunk. For identical results
            to the sync version, set chunk_size >= n.

        Note:
            Requires pyarrow to be installed: pip install pyarrow

        Args:
            n: Number of records to generate.
            schema: Dictionary mapping field names to type specifications.
            chunk_size: Number of records per chunk (default: 10,000).

        Returns:
            A coroutine that resolves to a PyArrow RecordBatch.

        Raises:
            ValueError: If n exceeds the maximum batch size or schema is invalid.
            ImportError: If pyarrow is not installed.
        """
        ...

    # Custom provider methods
    def add_provider(self, name: str, options: list[str]) -> None:
        """Register a custom provider with uniform random selection.

        Args:
            name: The provider name (must not conflict with built-in types)
            options: List of string options to choose from

        Raises:
            ValueError: If name conflicts with built-in type or options is empty
        """
        ...

    def add_weighted_provider(self, name: str, weighted_options: list[tuple[str, int]]) -> None:
        """Register a custom provider with weighted random selection.

        Args:
            name: The provider name
            weighted_options: List of (value, weight) tuples. Higher weights = more likely.

        Raises:
            ValueError: If name conflicts, options empty, or weights invalid
        """
        ...

    def remove_provider(self, name: str) -> bool:
        """Remove a custom provider.

        Args:
            name: The provider name to remove

        Returns:
            True if provider was removed, False if it didn't exist
        """
        ...

    def has_provider(self, name: str) -> bool:
        """Check if a custom provider exists.

        Args:
            name: The provider name to check

        Returns:
            True if provider exists, False otherwise
        """
        ...

    def list_providers(self) -> list[str]:
        """List all registered custom provider names.

        Returns:
            List of registered custom provider names
        """
        ...

    def generate(self, name: str) -> str:
        """Generate a single value from a custom provider.

        Args:
            name: The custom provider name

        Returns:
            A randomly selected string from the provider's options

        Raises:
            ValueError: If provider doesn't exist
        """
        ...

    def generate_batch(self, name: str, n: int) -> list[str]:
        """Generate a batch of values from a custom provider.

        Args:
            name: The custom provider name
            n: Number of values to generate

        Returns:
            A list of randomly selected strings

        Raises:
            ValueError: If provider doesn't exist or n exceeds batch limit
        """
        ...
