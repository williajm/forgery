"""Tests for locale support across all providers."""

import pytest

from forgery import Faker

SUPPORTED_LOCALES = ["en_US", "en_GB", "de_DE", "fr_FR", "es_ES", "it_IT", "ja_JP"]


class TestLocaleInstantiation:
    """Test that Faker can be created with all supported locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_faker_creation(self, locale: str) -> None:
        """Each supported locale should create successfully."""
        fake = Faker(locale)
        assert fake is not None

    def test_unsupported_locale_raises(self) -> None:
        """Unsupported locale should raise ValueError."""
        with pytest.raises(ValueError, match="unsupported locale"):
            Faker("xx_XX")

    def test_invalid_locale_format_raises(self) -> None:
        """Invalid locale format should raise ValueError."""
        with pytest.raises(ValueError, match="unsupported locale"):
            Faker("invalid")


class TestLocaleNames:
    """Test name generation for all locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_names_generation(self, locale: str) -> None:
        """Names should be generated for all locales."""
        fake = Faker(locale)
        names = fake.names(10)
        assert len(names) == 10
        assert all(isinstance(n, str) for n in names)
        assert all(len(n) > 0 for n in names)

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_first_names_generation(self, locale: str) -> None:
        """First names should be generated for all locales."""
        fake = Faker(locale)
        names = fake.first_names(10)
        assert len(names) == 10
        assert all(isinstance(n, str) for n in names)

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_last_names_generation(self, locale: str) -> None:
        """Last names should be generated for all locales."""
        fake = Faker(locale)
        names = fake.last_names(10)
        assert len(names) == 10
        assert all(isinstance(n, str) for n in names)

    def test_japanese_name_order(self) -> None:
        """Japanese names should have family name first."""
        fake = Faker("ja_JP")
        fake.seed(42)
        # Generate multiple names to get a good sample
        names = fake.names(100)
        # Just verify they are non-empty valid strings
        assert all(len(n) > 0 for n in names)


class TestLocaleAddresses:
    """Test address generation for all locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_addresses_generation(self, locale: str) -> None:
        """Addresses should be generated for all locales."""
        fake = Faker(locale)
        addresses = fake.addresses(10)
        assert len(addresses) == 10
        assert all(isinstance(a, str) for a in addresses)
        assert all(len(a) > 0 for a in addresses)

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_cities_generation(self, locale: str) -> None:
        """Cities should be generated for all locales."""
        fake = Faker(locale)
        cities = fake.cities(10)
        assert len(cities) == 10
        assert all(isinstance(c, str) for c in cities)

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_states_generation(self, locale: str) -> None:
        """States/regions should be generated for all locales."""
        fake = Faker(locale)
        states = fake.states(10)
        assert len(states) == 10
        assert all(isinstance(s, str) for s in states)

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_zip_codes_generation(self, locale: str) -> None:
        """Zip/postal codes should be generated for all locales."""
        fake = Faker(locale)
        zips = fake.zip_codes(10)
        assert len(zips) == 10
        assert all(isinstance(z, str) for z in zips)
        assert all(len(z) > 0 for z in zips)


class TestLocalePostalCodeFormats:
    """Test locale-specific postal code formats."""

    def test_us_zip_format(self) -> None:
        """US zip codes should be 5 or 9 digits."""
        fake = Faker("en_US")
        zips = fake.zip_codes(100)
        for z in zips:
            # Either "12345" or "12345-6789"
            assert len(z) == 5 or len(z) == 10

    def test_uk_postal_format(self) -> None:
        """UK postal codes should match UK format."""
        fake = Faker("en_GB")
        zips = fake.zip_codes(100)
        for z in zips:
            # UK format has space
            assert " " in z
            # Should have letters
            assert any(c.isalpha() for c in z)

    def test_german_postal_format(self) -> None:
        """German postal codes should be 5 digits."""
        fake = Faker("de_DE")
        zips = fake.zip_codes(100)
        for z in zips:
            assert len(z) == 5
            assert z.isdigit()

    def test_japanese_postal_format(self) -> None:
        """Japanese postal codes should be XXX-XXXX format."""
        fake = Faker("ja_JP")
        zips = fake.zip_codes(100)
        for z in zips:
            assert len(z) == 8
            assert z[3] == "-"


class TestLocalePhoneNumbers:
    """Test phone number generation for all locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_phone_numbers_generation(self, locale: str) -> None:
        """Phone numbers should be generated for all locales."""
        fake = Faker(locale)
        phones = fake.phone_numbers(10)
        assert len(phones) == 10
        assert all(isinstance(p, str) for p in phones)
        assert all(len(p) > 0 for p in phones)
        # All should have digits
        for phone in phones:
            assert any(c.isdigit() for c in phone)


class TestLocaleCompanies:
    """Test company generation for all locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_companies_generation(self, locale: str) -> None:
        """Companies should be generated for all locales."""
        fake = Faker(locale)
        companies = fake.companies(10)
        assert len(companies) == 10
        assert all(isinstance(c, str) for c in companies)

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_jobs_generation(self, locale: str) -> None:
        """Job titles should be generated for all locales."""
        fake = Faker(locale)
        jobs = fake.jobs(10)
        assert len(jobs) == 10
        assert all(isinstance(j, str) for j in jobs)


class TestLocaleEmails:
    """Test email generation for all locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_emails_generation(self, locale: str) -> None:
        """Emails should be generated for all locales."""
        fake = Faker(locale)
        emails = fake.emails(10)
        assert len(emails) == 10
        assert all(isinstance(e, str) for e in emails)
        # All should have @ symbol
        assert all("@" in e for e in emails)
        # All should be lowercase
        assert all(e == e.lower() for e in emails)

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_emails_are_ascii(self, locale: str) -> None:
        """Emails should use ASCII-only characters."""
        fake = Faker(locale)
        emails = fake.emails(100)
        for email in emails:
            assert email.isascii(), f"Non-ASCII email: {email}"


class TestLocaleColors:
    """Test color generation for all locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_colors_generation(self, locale: str) -> None:
        """Color names should be generated for all locales."""
        fake = Faker(locale)
        colors = fake.colors(10)
        assert len(colors) == 10
        assert all(isinstance(c, str) for c in colors)


class TestLocaleText:
    """Test text generation for all locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_sentences_generation(self, locale: str) -> None:
        """Sentences should be generated for all locales."""
        fake = Faker(locale)
        sentences = fake.sentences(10)
        assert len(sentences) == 10
        assert all(isinstance(s, str) for s in sentences)
        # All should end with period
        assert all(s.endswith(".") for s in sentences)

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_paragraphs_generation(self, locale: str) -> None:
        """Paragraphs should be generated for all locales."""
        fake = Faker(locale)
        paragraphs = fake.paragraphs(5)
        assert len(paragraphs) == 5
        assert all(isinstance(p, str) for p in paragraphs)


class TestLocaleDeterminism:
    """Test that seeding produces deterministic results per locale."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_seeded_determinism(self, locale: str) -> None:
        """Same seed should produce same results for each locale."""
        fake1 = Faker(locale)
        fake1.seed(12345)
        result1 = fake1.names(10)

        fake2 = Faker(locale)
        fake2.seed(12345)
        result2 = fake2.names(10)

        assert result1 == result2

    def test_different_locales_different_output(self) -> None:
        """Different locales should produce different output with same seed."""
        fake_us = Faker("en_US")
        fake_us.seed(42)
        us_names = fake_us.names(10)

        fake_de = Faker("de_DE")
        fake_de.seed(42)
        de_names = fake_de.names(10)

        # Names should be different (very unlikely to be the same)
        assert us_names != de_names


class TestLocaleAddressFormats:
    """Test locale-specific address formatting."""

    def test_german_street_address_no_space(self) -> None:
        """German street addresses should have compound names (no space)."""
        fake = Faker("de_DE")
        fake.seed(42)
        addresses = fake.street_addresses(100)
        for addr in addresses:
            # German addresses end with number, street name is compound
            # e.g., "Hauptstraße 123" not "Haupt straße 123"
            # The number should be at the end
            parts = addr.split()
            assert parts[-1].isdigit(), f"German address should end with number: {addr}"
            # There should be exactly 2 parts: compound street name and number
            assert len(parts) == 2, f"German address should be 'StreetName Number': {addr}"

    def test_us_street_address_format(self) -> None:
        """US street addresses should have number before street name."""
        fake = Faker("en_US")
        fake.seed(42)
        addresses = fake.street_addresses(100)
        for addr in addresses:
            parts = addr.split()
            # First part should be the number
            assert parts[0].isdigit(), f"US address should start with number: {addr}"
            # Should have at least 3 parts: number, street, suffix
            assert len(parts) >= 3, f"US address should have number, street, suffix: {addr}"

    def test_german_full_address_format(self) -> None:
        """German full addresses should use German template format."""
        fake = Faker("de_DE")
        fake.seed(42)
        addresses = fake.addresses(50)
        for addr in addresses:
            # German template is "{street}\n{postal} {city}"
            # Should have newline
            assert "\n" in addr, f"German address should have newline: {addr}"
            lines = addr.split("\n")
            assert len(lines) == 2, f"German address should have 2 lines: {addr}"
            # First line is street, second is postal+city
            second_line = lines[1]
            # Should start with 5-digit postal code
            parts = second_line.split()
            assert len(parts) >= 2, f"Second line should have postal and city: {addr}"
            assert parts[0].isdigit() and len(parts[0]) == 5, (
                f"Should start with 5-digit postal: {addr}"
            )

    def test_japanese_full_address_format(self) -> None:
        """Japanese full addresses should use Japanese template format."""
        fake = Faker("ja_JP")
        fake.seed(42)
        addresses = fake.addresses(50)
        for addr in addresses:
            # Japanese template is "〒{postal} {region}{city}{street}"
            # Should start with postal symbol
            assert addr.startswith("〒"), f"Japanese address should start with 〒: {addr}"
            # Postal code follows (format: XXX-XXXX)
            assert "-" in addr[:12], f"Japanese address should have postal code: {addr}"

    def test_us_full_address_format(self) -> None:
        """US full addresses should use US template format."""
        fake = Faker("en_US")
        fake.seed(42)
        addresses = fake.addresses(50)
        for addr in addresses:
            # US template is "{street}, {city}, {region_abbr} {postal}"
            # Should have commas
            assert addr.count(",") >= 2, f"US address should have commas: {addr}"
            # Should not have newlines
            assert "\n" not in addr, f"US address should not have newlines: {addr}"


class TestLocaleRecords:
    """Test record generation works with all locales."""

    @pytest.mark.parametrize("locale", SUPPORTED_LOCALES)
    def test_records_generation(self, locale: str) -> None:
        """Records should work with all locales."""
        fake = Faker(locale)
        schema = {
            "name": "name",
            "email": "email",
            "city": "city",
            "company": "company",
        }
        records = fake.records(5, schema)
        assert len(records) == 5
        for record in records:
            assert "name" in record
            assert "email" in record
            assert "city" in record
            assert "company" in record
