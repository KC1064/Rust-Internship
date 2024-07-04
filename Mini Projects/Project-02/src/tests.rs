#[cfg(test)]
mod tests {
    use crate::BankAccount;

    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(123456, 1000.0);
        assert_eq!(account.account_number, 123456);
        assert_eq!(account.balance, 1000.0);
    }

    #[test]
    fn test_get_balance() {
        let account = BankAccount::new(123456, 1000.0);
        assert_eq!(account.get_balance(), 1000.0);
    }

    #[test]
    fn test_deposit() {
        let account = BankAccount::new(123456, 1000.0);
        let updated_account = account.deposit(500.0);
        assert_eq!(updated_account.get_balance(), 1500.0);
    }

    #[test]
    fn test_withdraw() {
        let account = BankAccount::new(123456, 1000.0);
        let updated_account = account.withdraw(500.0);
        assert_eq!(updated_account.get_balance(), 500.0);
    }

    #[test]
    fn test_withdraw_insufficient_funds() {
        let account = BankAccount::new(123456, 1000.0);
        let updated_account = account.withdraw(1500.0);
        assert_eq!(updated_account.get_balance(), 1000.0); // Balance remains unchanged
    }

    #[test]
    fn test_transfer() {
        let account1 = BankAccount::new(123456, 1000.0);
        let account2 = BankAccount::new(654321, 500.0);
        let (updated_account1, updated_account2) = account1.transfer(account2, 200.0);
        assert_eq!(updated_account1.get_balance(), 800.0);
        assert_eq!(updated_account2.get_balance(), 700.0);
    }
}