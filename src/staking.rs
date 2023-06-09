#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const BLOCKS_IN_YEAR: u64 = 60 * 60 * 24 * 365 / 6;
pub const MAX_PRECISION: u64 = 1_000_000; //1000000

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct StakingPosition<M: ManagedTypeApi> {
    pub stake_amount: BigUint<M>,
    pub last_action_block: u64,
}

#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self, base_distribution: u64) {
        self.base_distribution().set(base_distribution);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        let payment_amount = self.call_value().egld_value().clone_value();
        require!(payment_amount > 0, "Must pay more than 0");

        let caller = self.blockchain().get_caller();
        let stake_mapper_amount = self.staking_amount(&caller);
        let stake_mapper_block = self.staking_block(&caller);

        let is_new_user = self.staked_addresses().insert(caller.clone());
        let (mut staking_amount, mut staking_block) = if !is_new_user {
            (stake_mapper_amount.get(), stake_mapper_block.get())
        } else {
            let current_block = self.blockchain().get_block_epoch();
            (BigUint::zero(), current_block)
        };

        self.claim_rewards_for_user(&caller, &staking_amount, &mut staking_block);
        staking_amount += payment_amount;

        stake_mapper_amount.set(&staking_amount);
        stake_mapper_block.set(&staking_block);
    }

    #[endpoint]
    fn unstake(&self, opt_unstake_amount: OptionalValue<BigUint>) {
        let caller = self.blockchain().get_caller();
        self.require_user_staked(&caller);

        let stake_mapper_amount = self.staking_amount(&caller);
        let stake_mapper_block = self.staking_block(&caller);
        let mut staking_amount = stake_mapper_amount.get();
        let mut staking_block = stake_mapper_block.get();

        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(amt) => amt,
            OptionalValue::None => staking_amount.clone(),
        };
        require!(
            unstake_amount > 0 && unstake_amount <= staking_amount,
            "Invalid unstake amount"
        );

        self.claim_rewards_for_user(&caller, &staking_amount, &mut staking_block);
        staking_amount -= &unstake_amount;

        if staking_amount > 0 {
            stake_mapper_amount.set(&staking_amount);
            stake_mapper_block.set(&staking_block);
        } else {
            stake_mapper_amount.clear();
            stake_mapper_block.clear();
            self.staked_addresses().swap_remove(&caller);
        }

        self.send().direct_egld(&caller, &unstake_amount);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        self.require_user_staked(&caller);

        let stake_mapper_amount = self.staking_amount(&caller);
        let stake_mapper_block = self.staking_block(&caller);
        let staking_amount = stake_mapper_amount.get();
        let mut staking_block = stake_mapper_block.get();
        self.claim_rewards_for_user(&caller, &staking_amount, &mut staking_block);

        stake_mapper_amount.set(&staking_amount);
        stake_mapper_block.set(&staking_block);
    }

    fn require_user_staked(&self, user: &ManagedAddress) {
        require!(self.staked_addresses().contains(user), "Must stake first");
    }
    fn claim_rewards_for_user(
        &self,
        user: &ManagedAddress,
        staking_amount: &BigUint,
        staking_block: &mut u64,
    ) {
        let reward_amount = self.calculate_rewards(staking_amount, staking_block);
        let current_block = self.blockchain().get_block_nonce();
        *staking_block = current_block;

        if reward_amount > 0 {
            self.send().direct_egld(user, &reward_amount);
        }
    }

    fn calculate_rewards(&self, staking_amount: &BigUint, staking_block: &u64) -> BigUint {
        let current_block = self.blockchain().get_block_nonce();
        if current_block <= *staking_block {
            return BigUint::zero();
        }

        let base_distribution = self.base_distribution().get();
        let block_diff = current_block - *staking_block;
        let time_diff = block_diff * 6;
        let second_per_year = BLOCKS_IN_YEAR * 6;

        staking_amount * base_distribution / MAX_PRECISION * time_diff / second_per_year
    }

    #[view(calculateRewardsForUser)]
    fn calculate_rewards_for_user(&self, addr: ManagedAddress) -> BigUint {
        let staking_amount = self.staking_amount(&addr).get();
        let staking_block = self.staking_block(&addr).get();
        self.calculate_rewards(&staking_amount, &staking_block)
    }

    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    // #[view(getStakingPosition)]
    // #[storage_mapper("stakingPosition")]
    // fn staking_position(
    //     &self,
    //     addr: &ManagedAddress,
    // ) -> SingleValueMapper<StakingPosition<Self::Api>>;

    #[view(getStakingAmount)]
    #[storage_mapper("stakingAmount")]
    fn staking_amount(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getStakingBlock)]
    #[storage_mapper("stakingBlock")]
    fn staking_block(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;

    #[view(getBaseDistribution)]
    #[storage_mapper("baseDistribution")]
    fn base_distribution(&self) -> SingleValueMapper<u64>;
}
