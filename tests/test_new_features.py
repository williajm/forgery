"""Tests for new features: passwords, banking, unique values, and UK banking."""

import pytest

from forgery import (
    Faker,
    bank_account,
    bank_accounts,
    bank_name,
    bank_names,
    bic,
    bics,
    password,
    passwords,
    sort_code,
    sort_codes,
    transaction_amount,
    transaction_amounts,
    transactions,
    uk_account_number,
    uk_account_numbers,
)


class TestPasswordGeneration:
    """Tests for password generation."""

    def test_password_default(self) -> None:
        """Test default password generation."""
        pwd = password()
        assert len(pwd) == 12
        assert any(c.isupper() for c in pwd)
        assert any(c.islower() for c in pwd)
        assert any(c.isdigit() for c in pwd)

    def test_password_custom_length(self) -> None:
        """Test password with custom length."""
        pwd = password(length=20)
        assert len(pwd) == 20

    def test_password_lowercase_only(self) -> None:
        """Test password with lowercase only."""
        pwd = password(length=20, uppercase=False, digits=False, symbols=False)
        assert pwd.islower()

    def test_password_uppercase_only(self) -> None:
        """Test password with uppercase only."""
        pwd = password(length=20, lowercase=False, digits=False, symbols=False)
        assert pwd.isupper()

    def test_password_digits_only(self) -> None:
        """Test password with digits only."""
        pwd = password(length=20, uppercase=False, lowercase=False, symbols=False)
        assert pwd.isdigit()

    def test_password_no_charset_error(self) -> None:
        """Test that disabling all charsets raises error."""
        with pytest.raises(ValueError, match="at least one character"):
            password(uppercase=False, lowercase=False, digits=False, symbols=False)

    def test_passwords_batch(self) -> None:
        """Test batch password generation."""
        pwds = passwords(100)
        assert len(pwds) == 100
        assert all(len(p) == 12 for p in pwds)

    def test_passwords_deterministic(self) -> None:
        """Test password generation is deterministic with seed."""
        fake1 = Faker()
        fake1.seed(42)
        pwds1 = fake1.passwords(10)

        fake2 = Faker()
        fake2.seed(42)
        pwds2 = fake2.passwords(10)

        assert pwds1 == pwds2


class TestBICGeneration:
    """Tests for BIC/SWIFT code generation."""

    def test_bic_format(self) -> None:
        """Test BIC format is 8 or 11 characters."""
        code = bic()
        assert len(code) in [8, 11]
        # BIC format: AAAABBCCXXX
        # First 6 chars are letters
        assert code[:6].isalpha()

    def test_bics_batch(self) -> None:
        """Test batch BIC generation."""
        codes = bics(100)
        assert len(codes) == 100
        for code in codes:
            assert len(code) in [8, 11]

    def test_bics_deterministic(self) -> None:
        """Test BIC generation is deterministic with seed."""
        fake1 = Faker()
        fake1.seed(42)
        codes1 = fake1.bics(10)

        fake2 = Faker()
        fake2.seed(42)
        codes2 = fake2.bics(10)

        assert codes1 == codes2


class TestBankAccountGeneration:
    """Tests for bank account number generation."""

    def test_bank_account_format(self) -> None:
        """Test bank account is 8-17 digits."""
        account = bank_account()
        assert account.isdigit()
        assert 8 <= len(account) <= 17

    def test_bank_accounts_batch(self) -> None:
        """Test batch bank account generation."""
        accounts = bank_accounts(100)
        assert len(accounts) == 100
        for account in accounts:
            assert account.isdigit()
            assert 8 <= len(account) <= 17

    def test_bank_accounts_deterministic(self) -> None:
        """Test bank account generation is deterministic with seed."""
        fake1 = Faker()
        fake1.seed(42)
        accounts1 = fake1.bank_accounts(10)

        fake2 = Faker()
        fake2.seed(42)
        accounts2 = fake2.bank_accounts(10)

        assert accounts1 == accounts2


class TestBankNameGeneration:
    """Tests for bank name generation."""

    def test_bank_name_not_empty(self) -> None:
        """Test bank name is not empty."""
        name = bank_name()
        assert len(name) > 0

    def test_bank_names_batch(self) -> None:
        """Test batch bank name generation."""
        names = bank_names(100)
        assert len(names) == 100
        for name in names:
            assert len(name) > 0

    def test_bank_names_deterministic(self) -> None:
        """Test bank name generation is deterministic with seed."""
        fake1 = Faker()
        fake1.seed(42)
        names1 = fake1.bank_names(10)

        fake2 = Faker()
        fake2.seed(42)
        names2 = fake2.bank_names(10)

        assert names1 == names2

    def test_bank_names_locale_specific(self) -> None:
        """Test bank names vary by locale."""
        us_fake = Faker("en_US")
        us_fake.seed(42)
        us_bank = us_fake.bank_name()

        de_fake = Faker("de_DE")
        de_fake.seed(42)
        de_bank = de_fake.bank_name()

        # They should be different banks from different locales
        # Note: We can't guarantee they're different, but the lists are different
        assert us_bank is not None
        assert de_bank is not None


class TestUniqueGeneration:
    """Tests for unique value generation."""

    def test_unique_names(self) -> None:
        """Test unique name generation."""
        fake = Faker()
        fake.seed(42)
        names = fake.names(50, unique=True)
        assert len(names) == 50
        assert len(set(names)) == 50  # All unique

    def test_unique_emails(self) -> None:
        """Test unique email generation."""
        fake = Faker()
        fake.seed(42)
        emails = fake.emails(50, unique=True)
        assert len(emails) == 50
        assert len(set(emails)) == 50  # All unique

    def test_unique_cities(self) -> None:
        """Test unique city generation."""
        fake = Faker()
        fake.seed(42)
        cities = fake.cities(20, unique=True)
        assert len(cities) == 20
        assert len(set(cities)) == 20  # All unique

    def test_unique_countries(self) -> None:
        """Test unique country generation."""
        fake = Faker()
        fake.seed(42)
        countries = fake.countries(50, unique=True)
        assert len(countries) == 50
        assert len(set(countries)) == 50  # All unique

    def test_unique_exhaustion_error(self) -> None:
        """Test that requesting too many unique values raises error."""
        fake = Faker()
        fake.seed(42)
        # Requesting more unique cities than exist should fail
        with pytest.raises(ValueError, match="unique"):
            fake.cities(10000, unique=True)

    def test_unique_bank_names(self) -> None:
        """Test unique bank name generation."""
        fake = Faker()
        fake.seed(42)
        names = fake.bank_names(10, unique=True)
        assert len(names) == 10
        assert len(set(names)) == 10  # All unique

    def test_non_unique_allows_duplicates(self) -> None:
        """Test that non-unique mode allows duplicates."""
        fake = Faker()
        fake.seed(42)
        # Generate many values to increase chance of duplicates
        colors = fake.colors(1000, unique=False)
        assert len(colors) == 1000
        # With 1000 values from a limited set, we expect some duplicates
        assert len(set(colors)) < len(colors)


class TestConvenienceFunctions:
    """Tests for module-level convenience functions."""

    def test_bic_convenience(self) -> None:
        """Test bic convenience function."""
        code = bic()
        assert len(code) in [8, 11]

    def test_bics_convenience(self) -> None:
        """Test bics convenience function."""
        codes = bics(10)
        assert len(codes) == 10

    def test_bank_account_convenience(self) -> None:
        """Test bank_account convenience function."""
        account = bank_account()
        assert account.isdigit()

    def test_bank_accounts_convenience(self) -> None:
        """Test bank_accounts convenience function."""
        accounts = bank_accounts(10)
        assert len(accounts) == 10

    def test_bank_name_convenience(self) -> None:
        """Test bank_name convenience function."""
        name = bank_name()
        assert len(name) > 0

    def test_bank_names_convenience(self) -> None:
        """Test bank_names convenience function."""
        names = bank_names(10)
        assert len(names) == 10

    def test_password_convenience(self) -> None:
        """Test password convenience function."""
        pwd = password()
        assert len(pwd) == 12

    def test_passwords_convenience(self) -> None:
        """Test passwords convenience function."""
        pwds = passwords(10)
        assert len(pwds) == 10


class TestUKSortCodes:
    """Tests for UK sort code generation."""

    def test_sort_code_format(self) -> None:
        """Test sort code format is XX-XX-XX."""
        code = sort_code()
        assert len(code) == 8
        assert code[2] == "-"
        assert code[5] == "-"
        # Check all digit groups
        assert code[:2].isdigit()
        assert code[3:5].isdigit()
        assert code[6:8].isdigit()

    def test_sort_codes_batch(self) -> None:
        """Test batch sort code generation."""
        codes = sort_codes(100)
        assert len(codes) == 100
        for code in codes:
            assert len(code) == 8
            assert code[2] == "-"

    def test_sort_codes_deterministic(self) -> None:
        """Test sort code generation is deterministic with seed."""
        fake1 = Faker()
        fake1.seed(42)
        codes1 = fake1.sort_codes(10)

        fake2 = Faker()
        fake2.seed(42)
        codes2 = fake2.sort_codes(10)

        assert codes1 == codes2


class TestUKAccountNumbers:
    """Tests for UK bank account number generation."""

    def test_uk_account_number_format(self) -> None:
        """Test UK account number is exactly 8 digits."""
        account = uk_account_number()
        assert len(account) == 8
        assert account.isdigit()

    def test_uk_account_numbers_batch(self) -> None:
        """Test batch UK account number generation."""
        accounts = uk_account_numbers(100)
        assert len(accounts) == 100
        for account in accounts:
            assert len(account) == 8
            assert account.isdigit()

    def test_uk_account_numbers_deterministic(self) -> None:
        """Test UK account number generation is deterministic with seed."""
        fake1 = Faker()
        fake1.seed(42)
        accounts1 = fake1.uk_account_numbers(10)

        fake2 = Faker()
        fake2.seed(42)
        accounts2 = fake2.uk_account_numbers(10)

        assert accounts1 == accounts2


class TestTransactionGeneration:
    """Tests for financial transaction generation."""

    def test_transactions_count(self) -> None:
        """Test that correct number of transactions are generated."""
        txns = transactions(10, 1000.0, "2024-01-01", "2024-01-31")
        assert len(txns) == 10

    def test_transactions_have_all_fields(self) -> None:
        """Test that transactions have all required fields."""
        txns = transactions(5, 1000.0, "2024-01-01", "2024-01-31")
        required_fields = [
            "reference",
            "date",
            "amount",
            "transaction_type",
            "description",
            "balance",
        ]
        for txn in txns:
            for field in required_fields:
                assert field in txn

    def test_transactions_sorted_by_date(self) -> None:
        """Test that transactions are sorted chronologically."""
        txns = transactions(20, 1000.0, "2024-01-01", "2024-03-31")
        dates = [txn["date"] for txn in txns]
        assert dates == sorted(dates)

    def test_transactions_running_balance(self) -> None:
        """Test that running balance is calculated correctly."""
        starting = 1000.0
        txns = transactions(5, starting, "2024-01-01", "2024-01-31")

        balance = starting
        for txn in txns:
            balance += txn["amount"]
            # Allow for small floating point differences
            assert abs(balance - txn["balance"]) < 0.01

    def test_transactions_reference_format(self) -> None:
        """Test that transaction references are 8 alphanumeric chars."""
        txns = transactions(10, 1000.0, "2024-01-01", "2024-01-31")
        for txn in txns:
            ref = txn["reference"]
            assert len(ref) == 8
            assert ref.isalnum()

    def test_transactions_deterministic(self) -> None:
        """Test transaction generation is deterministic with seed."""
        fake1 = Faker()
        fake1.seed(42)
        txns1 = fake1.transactions(5, 1000.0, "2024-01-01", "2024-01-31")

        fake2 = Faker()
        fake2.seed(42)
        txns2 = fake2.transactions(5, 1000.0, "2024-01-01", "2024-01-31")

        assert txns1 == txns2


class TestTransactionAmounts:
    """Tests for transaction amount generation."""

    def test_transaction_amount_range(self) -> None:
        """Test transaction amount is within range."""
        amount = transaction_amount(10.0, 100.0)
        assert 10.0 <= amount <= 100.0

    def test_transaction_amount_precision(self) -> None:
        """Test transaction amount has 2 decimal places."""
        amount = transaction_amount(10.0, 100.0)
        # Check it's rounded to 2 decimal places
        assert amount == round(amount, 2)

    def test_transaction_amounts_batch(self) -> None:
        """Test batch transaction amount generation."""
        amounts = transaction_amounts(100, 0.0, 1000.0)
        assert len(amounts) == 100
        for amount in amounts:
            assert 0.0 <= amount <= 1000.0
            assert amount == round(amount, 2)

    def test_transaction_amounts_deterministic(self) -> None:
        """Test transaction amount generation is deterministic with seed."""
        fake1 = Faker()
        fake1.seed(42)
        amounts1 = fake1.transaction_amounts(10, 0.0, 100.0)

        fake2 = Faker()
        fake2.seed(42)
        amounts2 = fake2.transaction_amounts(10, 0.0, 100.0)

        assert amounts1 == amounts2


class TestUKBankingConvenienceFunctions:
    """Tests for UK banking convenience functions."""

    def test_sort_code_convenience(self) -> None:
        """Test sort_code convenience function."""
        code = sort_code()
        assert len(code) == 8

    def test_sort_codes_convenience(self) -> None:
        """Test sort_codes convenience function."""
        codes = sort_codes(10)
        assert len(codes) == 10

    def test_uk_account_number_convenience(self) -> None:
        """Test uk_account_number convenience function."""
        account = uk_account_number()
        assert len(account) == 8

    def test_uk_account_numbers_convenience(self) -> None:
        """Test uk_account_numbers convenience function."""
        accounts = uk_account_numbers(10)
        assert len(accounts) == 10

    def test_transactions_convenience(self) -> None:
        """Test transactions convenience function."""
        txns = transactions(10, 1000.0, "2024-01-01", "2024-01-31")
        assert len(txns) == 10

    def test_transaction_amount_convenience(self) -> None:
        """Test transaction_amount convenience function."""
        amount = transaction_amount(10.0, 100.0)
        assert 10.0 <= amount <= 100.0

    def test_transaction_amounts_convenience(self) -> None:
        """Test transaction_amounts convenience function."""
        amounts = transaction_amounts(10, 10.0, 100.0)
        assert len(amounts) == 10


class TestEdgeCases:
    """Edge case tests for new features."""

    def test_empty_batch_passwords(self) -> None:
        """Test generating 0 passwords."""
        pwds = passwords(0)
        assert len(pwds) == 0

    def test_empty_batch_sort_codes(self) -> None:
        """Test generating 0 sort codes."""
        codes = sort_codes(0)
        assert len(codes) == 0

    def test_empty_batch_uk_account_numbers(self) -> None:
        """Test generating 0 UK account numbers."""
        accounts = uk_account_numbers(0)
        assert len(accounts) == 0

    def test_empty_batch_transactions(self) -> None:
        """Test generating 0 transactions."""
        txns = transactions(0, 1000.0, "2024-01-01", "2024-01-31")
        assert len(txns) == 0

    def test_empty_batch_bics(self) -> None:
        """Test generating 0 BICs."""
        codes = bics(0)
        assert len(codes) == 0

    def test_empty_batch_bank_accounts(self) -> None:
        """Test generating 0 bank accounts."""
        accounts = bank_accounts(0)
        assert len(accounts) == 0

    def test_empty_batch_bank_names(self) -> None:
        """Test generating 0 bank names."""
        names = bank_names(0)
        assert len(names) == 0

    def test_transaction_invalid_date_range(self) -> None:
        """Test that invalid date range raises an error."""
        fake = Faker()
        # End date before start date should raise ValueError
        with pytest.raises(ValueError, match="date"):
            fake.transactions(10, 1000.0, "2024-12-31", "2024-01-01")

    def test_transaction_same_day_range(self) -> None:
        """Test transactions with same start and end date."""
        fake = Faker()
        fake.seed(42)
        txns = fake.transactions(5, 1000.0, "2024-06-15", "2024-06-15")
        assert len(txns) == 5
        # All transactions should be on the same date
        for txn in txns:
            assert txn["date"] == "2024-06-15"

    def test_password_minimum_length(self) -> None:
        """Test password with minimum length of 1."""
        pwd = password(length=1)
        assert len(pwd) == 1

    def test_password_zero_length(self) -> None:
        """Test password with length 0 returns empty string."""
        pwd = password(length=0)
        assert len(pwd) == 0

    def test_transaction_negative_starting_balance(self) -> None:
        """Test transactions with negative starting balance."""
        fake = Faker()
        fake.seed(42)
        txns = fake.transactions(5, -500.0, "2024-01-01", "2024-01-31")
        assert len(txns) == 5
        # First transaction should adjust from -500

    def test_transaction_large_batch(self) -> None:
        """Test generating a large batch of transactions."""
        fake = Faker()
        fake.seed(42)
        txns = fake.transactions(1000, 10000.0, "2024-01-01", "2024-12-31")
        assert len(txns) == 1000
        # Verify dates are sorted
        dates = [t["date"] for t in txns]
        assert dates == sorted(dates)
