"""Tests for Phase 2 providers."""

import re

import pytest

import forgery
from forgery import Faker


class TestFloatGeneration:
    """Tests for float generation."""

    def test_float_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.float(0.0, 1.0)
        assert isinstance(val, float)
        assert 0.0 <= val <= 1.0

    def test_floats_batch(self):
        fake = Faker()
        fake.seed(42)
        vals = fake.floats(100, 0.0, 100.0)
        assert len(vals) == 100
        for v in vals:
            assert isinstance(v, float)
            assert 0.0 <= v <= 100.0

    def test_float_convenience(self):
        forgery.seed(42)
        val = forgery.float_(0.0, 1.0)
        assert isinstance(val, float)
        assert 0.0 <= val <= 1.0

    def test_floats_convenience(self):
        forgery.seed(42)
        vals = forgery.floats(10, 0.0, 1.0)
        assert len(vals) == 10


class TestHashGeneration:
    """Tests for MD5 and SHA256 generation."""

    def test_md5_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.md5()
        assert len(val) == 32
        assert all(c in '0123456789abcdef' for c in val)

    def test_md5_batch(self):
        fake = Faker()
        fake.seed(42)
        vals = fake.md5s(100)
        assert len(vals) == 100
        for v in vals:
            assert len(v) == 32

    def test_sha256_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.sha256()
        assert len(val) == 64
        assert all(c in '0123456789abcdef' for c in val)

    def test_sha256_batch(self):
        fake = Faker()
        fake.seed(42)
        vals = fake.sha256s(100)
        assert len(vals) == 100
        for v in vals:
            assert len(v) == 64

    def test_hash_convenience(self):
        forgery.seed(42)
        assert len(forgery.md5()) == 32
        assert len(forgery.sha256()) == 64
        assert len(forgery.md5s(5)) == 5
        assert len(forgery.sha256s(5)) == 5


class TestColorGeneration:
    """Tests for color generation."""

    def test_color_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.color()
        assert isinstance(val, str)
        assert len(val) > 0

    def test_hex_color_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.hex_color()
        assert val.startswith('#')
        assert len(val) == 7
        assert all(c in '0123456789abcdef' for c in val[1:])

    def test_rgb_color_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.rgb_color()
        assert isinstance(val, tuple)
        assert len(val) == 3
        for c in val:
            assert 0 <= c <= 255

    def test_color_batch(self):
        fake = Faker()
        fake.seed(42)
        assert len(fake.colors(50)) == 50
        assert len(fake.hex_colors(50)) == 50
        assert len(fake.rgb_colors(50)) == 50

    def test_color_convenience(self):
        forgery.seed(42)
        assert isinstance(forgery.color(), str)
        assert forgery.hex_color().startswith('#')
        assert len(forgery.rgb_color()) == 3
        assert len(forgery.colors(5)) == 5
        assert len(forgery.hex_colors(5)) == 5
        assert len(forgery.rgb_colors(5)) == 5


class TestDateTimeGeneration:
    """Tests for date and time generation."""

    def test_date_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.date("2020-01-01", "2020-12-31")
        assert re.match(r'\d{4}-\d{2}-\d{2}', val)
        assert val.startswith('2020-')

    def test_dates_batch(self):
        fake = Faker()
        fake.seed(42)
        vals = fake.dates(100, "2020-01-01", "2020-12-31")
        assert len(vals) == 100
        for v in vals:
            assert v.startswith('2020-')

    def test_date_of_birth(self):
        fake = Faker()
        fake.seed(42)
        val = fake.date_of_birth(18, 65)
        assert re.match(r'\d{4}-\d{2}-\d{2}', val)

    def test_datetime_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.datetime("2020-01-01", "2020-12-31")
        assert 'T' in val
        assert val.startswith('2020-')

    def test_date_convenience(self):
        forgery.seed(42)
        assert forgery.date().count('-') == 2
        assert len(forgery.dates(5)) == 5
        assert forgery.date_of_birth().count('-') == 2
        assert len(forgery.dates_of_birth(5)) == 5
        assert 'T' in forgery.datetime_()
        assert len(forgery.datetimes(5)) == 5


class TestTextGeneration:
    """Tests for text generation."""

    def test_sentence_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.sentence(10)
        assert isinstance(val, str)
        assert val.endswith('.')
        assert val[0].isupper()

    def test_paragraph_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.paragraph(5)
        assert isinstance(val, str)
        assert len(val) > 0

    def test_text_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.text(50, 100)
        assert 50 <= len(val) <= 100

    def test_text_batch(self):
        fake = Faker()
        fake.seed(42)
        assert len(fake.sentences(50)) == 50
        assert len(fake.paragraphs(50)) == 50
        assert len(fake.texts(50)) == 50

    def test_text_convenience(self):
        forgery.seed(42)
        assert forgery.sentence().endswith('.')
        assert len(forgery.sentences(5)) == 5
        assert isinstance(forgery.paragraph(), str)
        assert len(forgery.paragraphs(5)) == 5
        assert isinstance(forgery.text(), str)
        assert len(forgery.texts(5)) == 5


class TestAddressGeneration:
    """Tests for address generation."""

    def test_street_address(self):
        fake = Faker()
        fake.seed(42)
        val = fake.street_address()
        assert isinstance(val, str)
        # Should have a number and a street name
        assert any(c.isdigit() for c in val)

    def test_city(self):
        fake = Faker()
        fake.seed(42)
        val = fake.city()
        assert isinstance(val, str)
        assert len(val) > 0

    def test_state(self):
        fake = Faker()
        fake.seed(42)
        val = fake.state()
        assert isinstance(val, str)
        assert len(val) > 0

    def test_country(self):
        fake = Faker()
        fake.seed(42)
        val = fake.country()
        assert isinstance(val, str)
        assert len(val) > 0

    def test_zip_code(self):
        fake = Faker()
        fake.seed(42)
        val = fake.zip_code()
        assert isinstance(val, str)
        # US zip code format
        assert len(val) >= 5

    def test_full_address(self):
        fake = Faker()
        fake.seed(42)
        val = fake.address()
        assert isinstance(val, str)
        # Should contain a comma
        assert ',' in val

    def test_address_batch(self):
        fake = Faker()
        fake.seed(42)
        assert len(fake.street_addresses(50)) == 50
        assert len(fake.cities(50)) == 50
        assert len(fake.states(50)) == 50
        assert len(fake.countries(50)) == 50
        assert len(fake.zip_codes(50)) == 50
        assert len(fake.addresses(50)) == 50

    def test_address_convenience(self):
        forgery.seed(42)
        assert isinstance(forgery.street_address(), str)
        assert isinstance(forgery.city(), str)
        assert isinstance(forgery.state(), str)
        assert isinstance(forgery.country(), str)
        assert isinstance(forgery.zip_code(), str)
        assert isinstance(forgery.address(), str)
        assert len(forgery.street_addresses(5)) == 5
        assert len(forgery.cities(5)) == 5
        assert len(forgery.states(5)) == 5
        assert len(forgery.countries(5)) == 5
        assert len(forgery.zip_codes(5)) == 5
        assert len(forgery.addresses(5)) == 5


class TestPhoneGeneration:
    """Tests for phone number generation."""

    def test_phone_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.phone_number()
        assert isinstance(val, str)
        # US format: (XXX) XXX-XXXX
        assert '(' in val and ')' in val and '-' in val

    def test_phone_batch(self):
        fake = Faker()
        fake.seed(42)
        vals = fake.phone_numbers(100)
        assert len(vals) == 100

    def test_phone_convenience(self):
        forgery.seed(42)
        assert '(' in forgery.phone_number()
        assert len(forgery.phone_numbers(5)) == 5


class TestCompanyGeneration:
    """Tests for company generation."""

    def test_company_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.company()
        assert isinstance(val, str)
        assert len(val) > 0

    def test_job_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.job()
        assert isinstance(val, str)
        assert len(val) > 0

    def test_catch_phrase_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.catch_phrase()
        assert isinstance(val, str)
        assert len(val) > 0

    def test_company_batch(self):
        fake = Faker()
        fake.seed(42)
        assert len(fake.companies(50)) == 50
        assert len(fake.jobs(50)) == 50
        assert len(fake.catch_phrases(50)) == 50

    def test_company_convenience(self):
        forgery.seed(42)
        assert isinstance(forgery.company(), str)
        assert isinstance(forgery.job(), str)
        assert isinstance(forgery.catch_phrase(), str)
        assert len(forgery.companies(5)) == 5
        assert len(forgery.jobs(5)) == 5
        assert len(forgery.catch_phrases(5)) == 5


class TestNetworkGeneration:
    """Tests for network-related generation."""

    def test_url_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.url()
        assert val.startswith('https://')

    def test_domain_name_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.domain_name()
        assert '.' in val

    def test_ipv4_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.ipv4()
        parts = val.split('.')
        assert len(parts) == 4
        for p in parts:
            assert 0 <= int(p) <= 255

    def test_ipv6_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.ipv6()
        parts = val.split(':')
        assert len(parts) == 8
        for p in parts:
            assert len(p) == 4

    def test_mac_address_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.mac_address()
        parts = val.split(':')
        assert len(parts) == 6
        for p in parts:
            assert len(p) == 2

    def test_network_batch(self):
        fake = Faker()
        fake.seed(42)
        assert len(fake.urls(50)) == 50
        assert len(fake.domain_names(50)) == 50
        assert len(fake.ipv4s(50)) == 50
        assert len(fake.ipv6s(50)) == 50
        assert len(fake.mac_addresses(50)) == 50

    def test_network_convenience(self):
        forgery.seed(42)
        assert forgery.url().startswith('https://')
        assert '.' in forgery.domain_name()
        assert len(forgery.ipv4().split('.')) == 4
        assert len(forgery.ipv6().split(':')) == 8
        assert len(forgery.mac_address().split(':')) == 6
        assert len(forgery.urls(5)) == 5
        assert len(forgery.domain_names(5)) == 5
        assert len(forgery.ipv4s(5)) == 5
        assert len(forgery.ipv6s(5)) == 5
        assert len(forgery.mac_addresses(5)) == 5


class TestEmailVariants:
    """Tests for safe and free email generation."""

    def test_safe_email_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.safe_email()
        assert '@' in val
        domain = val.split('@')[1]
        assert domain in ['example.com', 'example.org', 'example.net']

    def test_free_email_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.free_email()
        assert '@' in val
        domain = val.split('@')[1]
        assert domain in ['gmail.com', 'yahoo.com', 'hotmail.com', 'outlook.com',
                          'aol.com', 'icloud.com', 'mail.com', 'protonmail.com',
                          'zoho.com', 'yandex.com']

    def test_email_batch(self):
        fake = Faker()
        fake.seed(42)
        assert len(fake.safe_emails(50)) == 50
        assert len(fake.free_emails(50)) == 50

    def test_email_convenience(self):
        forgery.seed(42)
        assert '@example' in forgery.safe_email()
        assert '@' in forgery.free_email()
        assert len(forgery.safe_emails(5)) == 5
        assert len(forgery.free_emails(5)) == 5


class TestFinanceGeneration:
    """Tests for finance-related generation."""

    def test_credit_card_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.credit_card()
        assert isinstance(val, str)
        assert len(val) in [15, 16]
        assert val.isdigit()
        # Luhn validation
        assert validate_luhn(val)

    def test_iban_single(self):
        fake = Faker()
        fake.seed(42)
        val = fake.iban()
        assert isinstance(val, str)
        assert len(val) >= 15
        # First 2 chars are country code
        assert val[:2].isalpha() and val[:2].isupper()
        # Next 2 are check digits
        assert val[2:4].isdigit()
        # IBAN validation
        assert validate_iban(val)

    def test_finance_batch(self):
        fake = Faker()
        fake.seed(42)
        cards = fake.credit_cards(100)
        assert len(cards) == 100
        for c in cards:
            assert validate_luhn(c)

        ibans = fake.ibans(100)
        assert len(ibans) == 100
        for i in ibans:
            assert validate_iban(i)

    def test_finance_convenience(self):
        forgery.seed(42)
        assert validate_luhn(forgery.credit_card())
        assert validate_iban(forgery.iban())
        assert len(forgery.credit_cards(5)) == 5
        assert len(forgery.ibans(5)) == 5


class TestDeterminism:
    """Tests for deterministic generation across new providers."""

    def test_new_providers_deterministic(self):
        fake1 = Faker()
        fake2 = Faker()

        fake1.seed(12345)
        fake2.seed(12345)

        # Test all new providers are deterministic
        assert fake1.float(0, 1) == fake2.float(0, 1)
        assert fake1.md5() == fake2.md5()
        assert fake1.sha256() == fake2.sha256()
        assert fake1.color() == fake2.color()
        assert fake1.hex_color() == fake2.hex_color()
        assert fake1.rgb_color() == fake2.rgb_color()
        assert fake1.date() == fake2.date()
        assert fake1.sentence() == fake2.sentence()
        assert fake1.street_address() == fake2.street_address()
        assert fake1.city() == fake2.city()
        assert fake1.phone_number() == fake2.phone_number()
        assert fake1.company() == fake2.company()
        assert fake1.url() == fake2.url()
        assert fake1.ipv4() == fake2.ipv4()
        assert fake1.safe_email() == fake2.safe_email()
        assert fake1.credit_card() == fake2.credit_card()
        assert fake1.iban() == fake2.iban()


def validate_luhn(number: str) -> bool:
    """Validate a credit card number using the Luhn algorithm."""
    digits = [int(d) for d in number if d.isdigit()]
    if not digits:
        return False

    checksum = 0
    for i, d in enumerate(reversed(digits)):
        if i % 2 == 1:
            d *= 2
            if d > 9:
                d -= 9
        checksum += d

    return checksum % 10 == 0


def validate_iban(iban: str) -> bool:
    """Validate an IBAN using ISO 7064 Mod 97-10."""
    # Remove spaces and convert to uppercase
    clean = ''.join(iban.split()).upper()
    if len(clean) < 5:
        return False

    # Move first 4 chars to end
    rearranged = clean[4:] + clean[:4]

    # Convert letters to numbers
    numeric = ''
    for c in rearranged:
        if c.isdigit():
            numeric += c
        elif c.isalpha():
            numeric += str(ord(c) - ord('A') + 10)
        else:
            return False

    # Check mod 97
    return int(numeric) % 97 == 1
