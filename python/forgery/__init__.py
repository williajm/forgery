"""forgery - Fake data at the speed of Rust.

A high-performance fake data generation library for Python, powered by Rust.
Designed to be 50-100x faster than Faker for batch operations.

Example:
    >>> from forgery import fake
    >>> fake.seed(42)
    >>> names = fake.names(1000)  # Generate 1000 names in one call
    >>> email = fake.email()  # Single value convenience method

    >>> from forgery import Faker
    >>> my_fake = Faker()
    >>> my_fake.seed(123)
    >>> my_fake.emails(100)
"""

from forgery._forgery import Faker

__all__ = [
    # Core
    "Faker",
    "fake",
    "seed",
    # Names
    "name",
    "names",
    "first_name",
    "first_names",
    "last_name",
    "last_names",
    # Internet
    "email",
    "emails",
    "safe_email",
    "safe_emails",
    "free_email",
    "free_emails",
    # Numbers
    "integer",
    "integers",
    "float_",
    "floats",
    # Identifiers
    "uuid",
    "uuids",
    "md5",
    "md5s",
    "sha256",
    "sha256s",
    # Colors
    "color",
    "colors",
    "hex_color",
    "hex_colors",
    "rgb_color",
    "rgb_colors",
    # DateTime
    "date",
    "dates",
    "date_of_birth",
    "dates_of_birth",
    "datetime_",
    "datetimes",
    # Text
    "sentence",
    "sentences",
    "paragraph",
    "paragraphs",
    "text",
    "texts",
    # Address
    "street_address",
    "street_addresses",
    "city",
    "cities",
    "state",
    "states",
    "country",
    "countries",
    "zip_code",
    "zip_codes",
    "address",
    "addresses",
    # Phone
    "phone_number",
    "phone_numbers",
    # Company
    "company",
    "companies",
    "job",
    "jobs",
    "catch_phrase",
    "catch_phrases",
    # Network
    "url",
    "urls",
    "domain_name",
    "domain_names",
    "ipv4",
    "ipv4s",
    "ipv6",
    "ipv6s",
    "mac_address",
    "mac_addresses",
    # Finance
    "credit_card",
    "credit_cards",
    "iban",
    "ibans",
]

__version__ = "0.1.0"

# Default Faker instance for convenient access
fake: Faker = Faker()


def seed(value: int) -> None:
    """Seed the default Faker instance for deterministic output.

    Args:
        value: The seed value.

    Example:
        >>> from forgery import seed, names
        >>> seed(42)
        >>> names1 = names(10)
        >>> seed(42)
        >>> names2 = names(10)
        >>> assert names1 == names2
    """
    fake.seed(value)


def name() -> str:
    """Generate a single random full name.

    Returns:
        A full name (first + last).

    Example:
        >>> from forgery import name
        >>> print(name())
        John Smith
    """
    return fake.name()


def names(n: int) -> list[str]:
    """Generate a batch of random full names.

    Args:
        n: Number of names to generate.

    Returns:
        A list of full names.

    Example:
        >>> from forgery import names
        >>> batch = names(1000)
        >>> len(batch)
        1000
    """
    return fake.names(n)


def first_name() -> str:
    """Generate a single random first name.

    Returns:
        A first name.
    """
    return fake.first_name()


def first_names(n: int) -> list[str]:
    """Generate a batch of random first names.

    Args:
        n: Number of first names to generate.

    Returns:
        A list of first names.
    """
    return fake.first_names(n)


def last_name() -> str:
    """Generate a single random last name.

    Returns:
        A last name.
    """
    return fake.last_name()


def last_names(n: int) -> list[str]:
    """Generate a batch of random last names.

    Args:
        n: Number of last names to generate.

    Returns:
        A list of last names.
    """
    return fake.last_names(n)


def email() -> str:
    """Generate a single random email address.

    Returns:
        An email address.

    Example:
        >>> from forgery import email
        >>> print(email())
        john123@gmail.com
    """
    return fake.email()


def emails(n: int) -> list[str]:
    """Generate a batch of random email addresses.

    Args:
        n: Number of emails to generate.

    Returns:
        A list of email addresses.
    """
    return fake.emails(n)


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
    return fake.integer(min, max)


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
    return fake.integers(n, min, max)


def uuid() -> str:
    """Generate a single random UUID (version 4).

    Returns:
        A UUID string.

    Example:
        >>> from forgery import uuid
        >>> print(uuid())
        a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d
    """
    return fake.uuid()


def uuids(n: int) -> list[str]:
    """Generate a batch of random UUIDs (version 4).

    Args:
        n: Number of UUIDs to generate.

    Returns:
        A list of UUID strings.
    """
    return fake.uuids(n)


# === Float Generation ===


def float_(min: float = 0.0, max: float = 1.0) -> float:
    """Generate a single random float within a range.

    Note: Named float_ to avoid shadowing builtin float.
    """
    return fake.float(min, max)


def floats(n: int, min: float = 0.0, max: float = 1.0) -> list[float]:
    """Generate a batch of random floats within a range."""
    return fake.floats(n, min, max)


# === Hash Generation ===


def md5() -> str:
    """Generate a single random MD5 hash."""
    return fake.md5()


def md5s(n: int) -> list[str]:
    """Generate a batch of random MD5 hashes."""
    return fake.md5s(n)


def sha256() -> str:
    """Generate a single random SHA256 hash."""
    return fake.sha256()


def sha256s(n: int) -> list[str]:
    """Generate a batch of random SHA256 hashes."""
    return fake.sha256s(n)


# === Color Generation ===


def color() -> str:
    """Generate a single random color name."""
    return fake.color()


def colors(n: int) -> list[str]:
    """Generate a batch of random color names."""
    return fake.colors(n)


def hex_color() -> str:
    """Generate a single random hex color (#RRGGBB)."""
    return fake.hex_color()


def hex_colors(n: int) -> list[str]:
    """Generate a batch of random hex colors."""
    return fake.hex_colors(n)


def rgb_color() -> tuple[int, int, int]:
    """Generate a single random RGB color tuple."""
    return fake.rgb_color()


def rgb_colors(n: int) -> list[tuple[int, int, int]]:
    """Generate a batch of random RGB color tuples."""
    return fake.rgb_colors(n)


# === DateTime Generation ===


def date(start: str = "2000-01-01", end: str = "2030-12-31") -> str:
    """Generate a single random date (YYYY-MM-DD format)."""
    return fake.date(start, end)


def dates(n: int, start: str = "2000-01-01", end: str = "2030-12-31") -> list[str]:
    """Generate a batch of random dates."""
    return fake.dates(n, start, end)


def date_of_birth(min_age: int = 18, max_age: int = 80) -> str:
    """Generate a single random date of birth."""
    return fake.date_of_birth(min_age, max_age)


def dates_of_birth(n: int, min_age: int = 18, max_age: int = 80) -> list[str]:
    """Generate a batch of random dates of birth."""
    return fake.dates_of_birth(n, min_age, max_age)


def datetime_(start: str = "2000-01-01", end: str = "2030-12-31") -> str:
    """Generate a single random datetime (ISO 8601 format).

    Note: Named datetime_ to avoid shadowing the datetime module.
    """
    return fake.datetime(start, end)


def datetimes(n: int, start: str = "2000-01-01", end: str = "2030-12-31") -> list[str]:
    """Generate a batch of random datetimes."""
    return fake.datetimes(n, start, end)


# === Text Generation ===


def sentence(word_count: int = 10) -> str:
    """Generate a single random sentence."""
    return fake.sentence(word_count)


def sentences(n: int, word_count: int = 10) -> list[str]:
    """Generate a batch of random sentences."""
    return fake.sentences(n, word_count)


def paragraph(sentence_count: int = 5) -> str:
    """Generate a single random paragraph."""
    return fake.paragraph(sentence_count)


def paragraphs(n: int, sentence_count: int = 5) -> list[str]:
    """Generate a batch of random paragraphs."""
    return fake.paragraphs(n, sentence_count)


def text(min_chars: int = 50, max_chars: int = 200) -> str:
    """Generate a single random text block."""
    return fake.text(min_chars, max_chars)


def texts(n: int, min_chars: int = 50, max_chars: int = 200) -> list[str]:
    """Generate a batch of random text blocks."""
    return fake.texts(n, min_chars, max_chars)


# === Address Generation ===


def street_address() -> str:
    """Generate a single random street address."""
    return fake.street_address()


def street_addresses(n: int) -> list[str]:
    """Generate a batch of random street addresses."""
    return fake.street_addresses(n)


def city() -> str:
    """Generate a single random city name."""
    return fake.city()


def cities(n: int) -> list[str]:
    """Generate a batch of random city names."""
    return fake.cities(n)


def state() -> str:
    """Generate a single random state name."""
    return fake.state()


def states(n: int) -> list[str]:
    """Generate a batch of random state names."""
    return fake.states(n)


def country() -> str:
    """Generate a single random country name."""
    return fake.country()


def countries(n: int) -> list[str]:
    """Generate a batch of random country names."""
    return fake.countries(n)


def zip_code() -> str:
    """Generate a single random zip code."""
    return fake.zip_code()


def zip_codes(n: int) -> list[str]:
    """Generate a batch of random zip codes."""
    return fake.zip_codes(n)


def address() -> str:
    """Generate a single random full address."""
    return fake.address()


def addresses(n: int) -> list[str]:
    """Generate a batch of random full addresses."""
    return fake.addresses(n)


# === Phone Generation ===


def phone_number() -> str:
    """Generate a single random phone number."""
    return fake.phone_number()


def phone_numbers(n: int) -> list[str]:
    """Generate a batch of random phone numbers."""
    return fake.phone_numbers(n)


# === Company Generation ===


def company() -> str:
    """Generate a single random company name."""
    return fake.company()


def companies(n: int) -> list[str]:
    """Generate a batch of random company names."""
    return fake.companies(n)


def job() -> str:
    """Generate a single random job title."""
    return fake.job()


def jobs(n: int) -> list[str]:
    """Generate a batch of random job titles."""
    return fake.jobs(n)


def catch_phrase() -> str:
    """Generate a single random catch phrase."""
    return fake.catch_phrase()


def catch_phrases(n: int) -> list[str]:
    """Generate a batch of random catch phrases."""
    return fake.catch_phrases(n)


# === Network Generation ===


def url() -> str:
    """Generate a single random URL."""
    return fake.url()


def urls(n: int) -> list[str]:
    """Generate a batch of random URLs."""
    return fake.urls(n)


def domain_name() -> str:
    """Generate a single random domain name."""
    return fake.domain_name()


def domain_names(n: int) -> list[str]:
    """Generate a batch of random domain names."""
    return fake.domain_names(n)


def ipv4() -> str:
    """Generate a single random IPv4 address."""
    return fake.ipv4()


def ipv4s(n: int) -> list[str]:
    """Generate a batch of random IPv4 addresses."""
    return fake.ipv4s(n)


def ipv6() -> str:
    """Generate a single random IPv6 address."""
    return fake.ipv6()


def ipv6s(n: int) -> list[str]:
    """Generate a batch of random IPv6 addresses."""
    return fake.ipv6s(n)


def mac_address() -> str:
    """Generate a single random MAC address."""
    return fake.mac_address()


def mac_addresses(n: int) -> list[str]:
    """Generate a batch of random MAC addresses."""
    return fake.mac_addresses(n)


# === Email Variants ===


def safe_email() -> str:
    """Generate a single random safe email (example.com/org/net)."""
    return fake.safe_email()


def safe_emails(n: int) -> list[str]:
    """Generate a batch of random safe emails."""
    return fake.safe_emails(n)


def free_email() -> str:
    """Generate a single random free email (gmail.com, etc.)."""
    return fake.free_email()


def free_emails(n: int) -> list[str]:
    """Generate a batch of random free emails."""
    return fake.free_emails(n)


# === Finance Generation ===


def credit_card() -> str:
    """Generate a single random credit card number with valid Luhn checksum."""
    return fake.credit_card()


def credit_cards(n: int) -> list[str]:
    """Generate a batch of random credit card numbers."""
    return fake.credit_cards(n)


def iban() -> str:
    """Generate a single random IBAN with valid checksum."""
    return fake.iban()


def ibans(n: int) -> list[str]:
    """Generate a batch of random IBANs."""
    return fake.ibans(n)
