"""
Test coverage for identified gaps in the test suite.

This file contains tests to fill coverage gaps identified in the codebase analysis.
"""

import re

import pytest

import forgery
from forgery import Faker


class TestFloatRangeErrorHandling:
    """Test error handling for float generation with invalid ranges."""

    def test_float_min_greater_than_max_single(self):
        """Test that float() raises ValueError when min > max."""
        f = Faker()
        with pytest.raises(ValueError) as excinfo:
            f.float(10.0, 5.0)
        assert "range" in str(excinfo.value).lower() or "10" in str(excinfo.value)

    def test_float_min_greater_than_max_batch(self):
        """Test that floats() raises ValueError when min > max."""
        f = Faker()
        with pytest.raises(ValueError) as excinfo:
            f.floats(100, 10.0, 5.0)
        assert "range" in str(excinfo.value).lower() or "10" in str(excinfo.value)

    def test_float_module_level_error(self):
        """Test module-level float_ with invalid range."""
        with pytest.raises(ValueError):
            forgery.float_(100.0, 50.0)

    def test_floats_module_level_error(self):
        """Test module-level floats with invalid range."""
        with pytest.raises(ValueError):
            forgery.floats(10, 100.0, 50.0)

    def test_float_negative_range_valid(self):
        """Test that negative ranges work correctly when min < max."""
        f = Faker()
        f.seed(42)
        result = f.float(-10.0, -5.0)
        assert -10.0 <= result <= -5.0

    def test_floats_negative_range_valid(self):
        """Test batch float generation with valid negative range."""
        f = Faker()
        f.seed(42)
        results = f.floats(100, -100.0, -50.0)
        for val in results:
            assert -100.0 <= val <= -50.0


class TestDateRangeErrorHandling:
    """Test error handling for date generation with invalid ranges."""

    def test_date_invalid_start_format(self):
        """Test that date() raises error for invalid start date format."""
        f = Faker()
        with pytest.raises(ValueError) as excinfo:
            f.date("not-a-date", "2023-12-31")
        assert "date" in str(excinfo.value).lower() or "invalid" in str(excinfo.value).lower()

    def test_date_invalid_end_format(self):
        """Test that date() raises error for invalid end date format."""
        f = Faker()
        with pytest.raises(ValueError) as excinfo:
            f.date("2020-01-01", "invalid")
        assert "date" in str(excinfo.value).lower() or "invalid" in str(excinfo.value).lower()

    def test_date_start_after_end(self):
        """Test that date() raises error when start > end."""
        f = Faker()
        with pytest.raises(ValueError) as excinfo:
            f.date("2025-12-31", "2020-01-01")
        assert "start" in str(excinfo.value).lower() or "before" in str(excinfo.value).lower()

    def test_dates_batch_invalid_range(self):
        """Test batch dates with invalid range."""
        f = Faker()
        with pytest.raises(ValueError):
            f.dates(10, "2025-01-01", "2020-01-01")

    def test_datetime_start_after_end(self):
        """Test that datetime() raises error when start > end."""
        f = Faker()
        with pytest.raises(ValueError):
            f.datetime("2025-12-31", "2020-01-01")

    def test_datetimes_batch_invalid_range(self):
        """Test batch datetimes with invalid range."""
        f = Faker()
        with pytest.raises(ValueError):
            f.datetimes(10, "2025-01-01", "2020-01-01")

    def test_date_of_birth_min_age_greater_than_max(self):
        """Test that date_of_birth raises error when min_age > max_age."""
        f = Faker()
        with pytest.raises(ValueError):
            f.date_of_birth(80, 18)

    def test_dates_of_birth_invalid_age_range(self):
        """Test batch dates_of_birth with invalid age range."""
        f = Faker()
        with pytest.raises(ValueError):
            f.dates_of_birth(10, 100, 20)


class TestPhase2BatchSizeLimits:
    """Test batch size limits for Phase 2 providers."""

    MAX_BATCH = 10_000_001  # Just over the limit

    def test_floats_exceeds_limit(self):
        """Test floats batch size limit."""
        f = Faker()
        with pytest.raises(ValueError) as excinfo:
            f.floats(self.MAX_BATCH)
        assert "batch" in str(excinfo.value).lower() or "limit" in str(excinfo.value).lower()

    def test_dates_exceeds_limit(self):
        """Test dates batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.dates(self.MAX_BATCH)

    def test_datetimes_exceeds_limit(self):
        """Test datetimes batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.datetimes(self.MAX_BATCH)

    def test_dates_of_birth_exceeds_limit(self):
        """Test dates_of_birth batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.dates_of_birth(self.MAX_BATCH)

    def test_sentences_exceeds_limit(self):
        """Test sentences batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.sentences(self.MAX_BATCH)

    def test_paragraphs_exceeds_limit(self):
        """Test paragraphs batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.paragraphs(self.MAX_BATCH)

    def test_texts_exceeds_limit(self):
        """Test texts batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.texts(self.MAX_BATCH)

    def test_colors_exceeds_limit(self):
        """Test colors batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.colors(self.MAX_BATCH)

    def test_hex_colors_exceeds_limit(self):
        """Test hex_colors batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.hex_colors(self.MAX_BATCH)

    def test_rgb_colors_exceeds_limit(self):
        """Test rgb_colors batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.rgb_colors(self.MAX_BATCH)

    def test_street_addresses_exceeds_limit(self):
        """Test street_addresses batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.street_addresses(self.MAX_BATCH)

    def test_cities_exceeds_limit(self):
        """Test cities batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.cities(self.MAX_BATCH)

    def test_states_exceeds_limit(self):
        """Test states batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.states(self.MAX_BATCH)

    def test_countries_exceeds_limit(self):
        """Test countries batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.countries(self.MAX_BATCH)

    def test_zip_codes_exceeds_limit(self):
        """Test zip_codes batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.zip_codes(self.MAX_BATCH)

    def test_addresses_exceeds_limit(self):
        """Test addresses batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.addresses(self.MAX_BATCH)

    def test_phone_numbers_exceeds_limit(self):
        """Test phone_numbers batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.phone_numbers(self.MAX_BATCH)

    def test_companies_exceeds_limit(self):
        """Test companies batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.companies(self.MAX_BATCH)

    def test_jobs_exceeds_limit(self):
        """Test jobs batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.jobs(self.MAX_BATCH)

    def test_catch_phrases_exceeds_limit(self):
        """Test catch_phrases batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.catch_phrases(self.MAX_BATCH)

    def test_urls_exceeds_limit(self):
        """Test urls batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.urls(self.MAX_BATCH)

    def test_domain_names_exceeds_limit(self):
        """Test domain_names batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.domain_names(self.MAX_BATCH)

    def test_ipv4s_exceeds_limit(self):
        """Test ipv4s batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.ipv4s(self.MAX_BATCH)

    def test_ipv6s_exceeds_limit(self):
        """Test ipv6s batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.ipv6s(self.MAX_BATCH)

    def test_mac_addresses_exceeds_limit(self):
        """Test mac_addresses batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.mac_addresses(self.MAX_BATCH)

    def test_safe_emails_exceeds_limit(self):
        """Test safe_emails batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.safe_emails(self.MAX_BATCH)

    def test_free_emails_exceeds_limit(self):
        """Test free_emails batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.free_emails(self.MAX_BATCH)

    def test_credit_cards_exceeds_limit(self):
        """Test credit_cards batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.credit_cards(self.MAX_BATCH)

    def test_ibans_exceeds_limit(self):
        """Test ibans batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.ibans(self.MAX_BATCH)

    def test_md5s_exceeds_limit(self):
        """Test md5s batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.md5s(self.MAX_BATCH)

    def test_sha256s_exceeds_limit(self):
        """Test sha256s batch size limit."""
        f = Faker()
        with pytest.raises(ValueError):
            f.sha256s(self.MAX_BATCH)


class TestCreditCardValidation:
    """Test credit card format and Luhn checksum validation from Python."""

    def _validate_luhn(self, number: str) -> bool:
        """Validate a credit card number using the Luhn algorithm."""
        digits = [int(c) for c in number if c.isdigit()]
        if not digits:
            return False

        total = 0
        double = False
        for digit in reversed(digits):
            if double:
                digit *= 2
                if digit > 9:
                    digit -= 9
            total += digit
            double = not double

        return total % 10 == 0

    def test_credit_card_luhn_valid(self):
        """Test that generated credit cards pass Luhn validation."""
        f = Faker()
        f.seed(42)
        for _ in range(100):
            card = f.credit_card()
            assert self._validate_luhn(card), f"Credit card failed Luhn: {card}"

    def test_credit_cards_batch_luhn_valid(self):
        """Test batch credit cards pass Luhn validation."""
        f = Faker()
        f.seed(42)
        cards = f.credit_cards(500)
        for card in cards:
            assert self._validate_luhn(card), f"Credit card failed Luhn: {card}"

    def test_credit_card_length(self):
        """Test credit card lengths are 15 or 16 digits."""
        f = Faker()
        f.seed(42)
        for _ in range(100):
            card = f.credit_card()
            assert len(card) in (15, 16), f"Invalid length: {card}"
            assert card.isdigit(), f"Non-digit character in: {card}"

    def test_credit_card_prefix_visa(self):
        """Test that some cards have Visa prefix (4)."""
        f = Faker()
        f.seed(42)
        cards = f.credit_cards(1000)
        visa_cards = [c for c in cards if c.startswith("4")]
        assert len(visa_cards) > 0, "Should generate some Visa cards"

    def test_credit_card_prefix_mastercard(self):
        """Test that some cards have Mastercard prefix (51-55)."""
        f = Faker()
        f.seed(42)
        cards = f.credit_cards(1000)
        mc_cards = [c for c in cards if c[:2] in ("51", "52", "53", "54", "55")]
        assert len(mc_cards) > 0, "Should generate some Mastercard cards"


class TestIBANValidation:
    """Test IBAN format and checksum validation from Python."""

    def _validate_iban(self, iban: str) -> bool:
        """Validate an IBAN using ISO 7064 Mod 97-10."""
        clean = iban.replace(" ", "").upper()
        if len(clean) < 5:
            return False

        # Move first 4 characters to end
        rearranged = clean[4:] + clean[:4]

        # Convert letters to numbers
        numeric = ""
        for c in rearranged:
            if c.isdigit():
                numeric += c
            elif c.isalpha():
                numeric += str(ord(c) - ord("A") + 10)
            else:
                return False

        # Calculate mod 97
        remainder = int(numeric) % 97
        return remainder == 1

    def test_iban_checksum_valid(self):
        """Test that generated IBANs pass checksum validation."""
        f = Faker()
        f.seed(42)
        for _ in range(100):
            iban = f.iban()
            assert self._validate_iban(iban), f"IBAN failed validation: {iban}"

    def test_ibans_batch_checksum_valid(self):
        """Test batch IBANs pass checksum validation."""
        f = Faker()
        f.seed(42)
        ibans = f.ibans(500)
        for iban in ibans:
            assert self._validate_iban(iban), f"IBAN failed validation: {iban}"

    def test_iban_format(self):
        """Test IBAN format: 2-letter country code + 2 check digits + BBAN."""
        f = Faker()
        f.seed(42)
        for _ in range(100):
            iban = f.iban()
            # Country code: 2 uppercase letters
            assert iban[:2].isalpha() and iban[:2].isupper()
            # Check digits: 2 digits
            assert iban[2:4].isdigit()
            # Rest: alphanumeric
            assert iban[4:].isalnum()


class TestTextEdgeCases:
    """Test edge cases for text generation."""

    def test_sentence_zero_words(self):
        """Test sentence with 0 words returns empty or minimal string."""
        f = Faker()
        f.seed(42)
        result = f.sentence(0)
        assert isinstance(result, str)

    def test_paragraph_zero_sentences(self):
        """Test paragraph with 0 sentences returns empty or minimal string."""
        f = Faker()
        f.seed(42)
        result = f.paragraph(0)
        assert isinstance(result, str)

    def test_text_zero_max_chars(self):
        """Test text with 0 max_chars returns empty string."""
        f = Faker()
        f.seed(42)
        result = f.text(0, 0)
        assert result == ""

    def test_text_min_equals_max(self):
        """Test text with min_chars == max_chars."""
        f = Faker()
        f.seed(42)
        result = f.text(50, 50)
        assert len(result) == 50

    def test_sentence_large_word_count(self):
        """Test sentence with large word count."""
        f = Faker()
        f.seed(42)
        result = f.sentence(100)
        words = result.rstrip(".").split()
        assert len(words) == 100


class TestNetworkFormatValidation:
    """Test network-related format validation."""

    def test_ipv4_format_valid(self):
        """Test IPv4 addresses have correct format."""
        f = Faker()
        f.seed(42)
        ipv4_pattern = re.compile(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$")
        for _ in range(100):
            ip = f.ipv4()
            assert ipv4_pattern.match(ip), f"Invalid IPv4 format: {ip}"
            octets = [int(o) for o in ip.split(".")]
            for octet in octets:
                assert 0 <= octet <= 255, f"Octet out of range: {ip}"

    def test_ipv6_format_valid(self):
        """Test IPv6 addresses have correct format."""
        f = Faker()
        f.seed(42)
        for _ in range(100):
            ip = f.ipv6()
            parts = ip.split(":")
            assert len(parts) == 8, f"IPv6 should have 8 groups: {ip}"
            for part in parts:
                assert len(part) == 4, f"Each group should be 4 chars: {ip}"
                assert all(c in "0123456789abcdef" for c in part)

    def test_mac_address_format_valid(self):
        """Test MAC addresses have correct format."""
        f = Faker()
        f.seed(42)
        mac_pattern = re.compile(r"^([0-9a-f]{2}:){5}[0-9a-f]{2}$")
        for _ in range(100):
            mac = f.mac_address()
            assert mac_pattern.match(mac), f"Invalid MAC format: {mac}"

    def test_url_format_valid(self):
        """Test URLs have correct format."""
        f = Faker()
        f.seed(42)
        for _ in range(100):
            url = f.url()
            assert url.startswith("https://"), f"URL should start with https://: {url}"
            assert "." in url, f"URL should have domain: {url}"


class TestColorFormatValidation:
    """Test color format validation."""

    def test_hex_color_format(self):
        """Test hex colors have correct format #RRGGBB."""
        f = Faker()
        f.seed(42)
        hex_pattern = re.compile(r"^#[0-9a-f]{6}$")
        for _ in range(100):
            color = f.hex_color()
            assert hex_pattern.match(color), f"Invalid hex color: {color}"

    def test_rgb_color_values(self):
        """Test RGB color values are in range 0-255."""
        f = Faker()
        f.seed(42)
        for _ in range(100):
            r, g, b = f.rgb_color()
            assert 0 <= r <= 255, f"Red out of range: {r}"
            assert 0 <= g <= 255, f"Green out of range: {g}"
            assert 0 <= b <= 255, f"Blue out of range: {b}"


class TestDeterminismComprehensive:
    """Comprehensive determinism tests for all providers."""

    def test_all_providers_deterministic(self):
        """Test that all providers produce deterministic output with same seed."""
        f1 = Faker()
        f2 = Faker()

        f1.seed(12345)
        f2.seed(12345)

        # Test all providers
        assert f1.name() == f2.name()
        assert f1.email() == f2.email()
        assert f1.integer(0, 1000) == f2.integer(0, 1000)
        assert f1.uuid() == f2.uuid()
        assert f1.float(0.0, 1.0) == f2.float(0.0, 1.0)
        assert f1.md5() == f2.md5()
        assert f1.sha256() == f2.sha256()
        assert f1.color() == f2.color()
        assert f1.hex_color() == f2.hex_color()
        assert f1.rgb_color() == f2.rgb_color()
        assert f1.date() == f2.date()
        assert f1.datetime() == f2.datetime()
        assert f1.sentence() == f2.sentence()
        assert f1.paragraph() == f2.paragraph()
        assert f1.text() == f2.text()
        assert f1.street_address() == f2.street_address()
        assert f1.city() == f2.city()
        assert f1.state() == f2.state()
        assert f1.country() == f2.country()
        assert f1.zip_code() == f2.zip_code()
        assert f1.address() == f2.address()
        assert f1.phone_number() == f2.phone_number()
        assert f1.company() == f2.company()
        assert f1.job() == f2.job()
        assert f1.catch_phrase() == f2.catch_phrase()
        assert f1.url() == f2.url()
        assert f1.domain_name() == f2.domain_name()
        assert f1.ipv4() == f2.ipv4()
        assert f1.ipv6() == f2.ipv6()
        assert f1.mac_address() == f2.mac_address()
        assert f1.safe_email() == f2.safe_email()
        assert f1.free_email() == f2.free_email()
        assert f1.credit_card() == f2.credit_card()
        assert f1.iban() == f2.iban()


class TestEmptyBatchesAllProviders:
    """Test that all providers handle empty batches correctly."""

    def test_all_batch_providers_empty(self):
        """Test all batch providers return empty list for n=0."""
        f = Faker()

        assert f.names(0) == []
        assert f.first_names(0) == []
        assert f.last_names(0) == []
        assert f.emails(0) == []
        assert f.integers(0) == []
        assert f.uuids(0) == []
        assert f.floats(0) == []
        assert f.md5s(0) == []
        assert f.sha256s(0) == []
        assert f.colors(0) == []
        assert f.hex_colors(0) == []
        assert f.rgb_colors(0) == []
        assert f.dates(0) == []
        assert f.datetimes(0) == []
        assert f.dates_of_birth(0) == []
        assert f.sentences(0) == []
        assert f.paragraphs(0) == []
        assert f.texts(0) == []
        assert f.street_addresses(0) == []
        assert f.cities(0) == []
        assert f.states(0) == []
        assert f.countries(0) == []
        assert f.zip_codes(0) == []
        assert f.addresses(0) == []
        assert f.phone_numbers(0) == []
        assert f.companies(0) == []
        assert f.jobs(0) == []
        assert f.catch_phrases(0) == []
        assert f.urls(0) == []
        assert f.domain_names(0) == []
        assert f.ipv4s(0) == []
        assert f.ipv6s(0) == []
        assert f.mac_addresses(0) == []
        assert f.safe_emails(0) == []
        assert f.free_emails(0) == []
        assert f.credit_cards(0) == []
        assert f.ibans(0) == []
