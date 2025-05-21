use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, BytesN, Env, Map, Symbol, TryFromVal,
    Val, Vec,
};

#[contracttype]
pub enum DataKey {
    TokenAddress,
    Members,
    Proposals,
    ProposalCount,
}

#[contracttype]
pub enum ProposalStatus {
    Active,
    Executed,
    Rejected,
}

#[contracttype]
pub struct Proposal {
    pub id: u32,                 // ID for each proposal in the DAO
    pub proposer: Address,       // Address of the member who created the proposal
    pub description: BytesN<32>, // description of the proposal
    pub target: Address, // Contract address of the token contract that the proposal will interact with if executed
    pub function: Symbol, // name of the function to call on the target contract
    pub parameters: Vec<Val>, // A vector containing the parameters to pass the function when the
    // proposal is executed
    pub votes_for: i128,        // total votes cast for the proposal
    pub votes_against: i128,    // total votes cast against the proposal
    pub status: ProposalStatus, // enum tracking the current state of the proposal
    pub deadline: u64,          // timestamp representing when voting closes
}

#[contracttype]
pub struct Member {
    pub address: Address,
    pub token_balance: i128,
    pub joined_timestamp: u64,
    pub voted_proposals: Vec<u32>,
}

#[contract]
pub struct MiniDao;

#[contractimpl]
impl MiniDao {
    // Initialize the DAO with a token contract
    pub fn initalize(env: Env, admin: Address, token_address: Address) {
        admin.require_auth();

        // Store the token address
        env.storage()
            .instance()
            .set(&DataKey::TokenAddress, &token_address);

        // Initialize empty members list
        env.storage()
            .instance()
            .set(&DataKey::Members, &Map::<Address, Member>::new(&env));

        // Initialize empty proposals list
        env.storage()
            .instance()
            .set(&DataKey::Proposals, &Map::<u32, Proposal>::new(&env));

        // Initialize proposal count
        env.storage().instance().set(&DataKey::ProposalCount, &0u32);
    }

    // Add a new member to the DAO and mint tokens for them
    pub fn add_member(env: Env, admin: Address, new_member: Address) {
        admin.require_auth();

        if env.storage().instance().has(&DataKey::Members) {
            panic!("Member already exists");
        }

        // Check if member already exists
        let mut members: Map<Address, Member> =
            env.storage().instance().get(&DataKey::Members).unwrap();

        // Mint tokens for the new member
        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddress)
            .unwrap();
        let token_client = token::Client::new(&env, &token_address);
        let token_amount: i128 = 100; // each member gets 100 tokens

        // The admin transfers tokens to the new member
        token_client.transfer(&admin, &new_member, &token_amount);

        // Add member to the list
        let member = Member {
            address: new_member.clone(),
            token_balance: token_amount,
            joined_timestamp: env.ledger().timestamp(),
            voted_proposals: Vec::new(&env),
        };

        members.set(new_member, member);
        env.storage().instance().set(&DataKey::Members, &members);
    }

    // Create a new proposal
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        description: BytesN<32>,
        target: Address,
        function: Symbol,
        parameters: Vec<Val>,
        deadline_in_seconds: u64,
    ) -> u32 {
        proposer.require_auth();

        // Check if proposer is a member
        if !env.storage().instance().has(&DataKey::Members) {
            panic!("Only members can create proposals");
        }

        // Get and increment the proposal count
        let mut proposal_count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap();
        proposal_count += 1;

        // calculate deadline
        let current_time = env.ledger().timestamp();
        let deadline = current_time + deadline_in_seconds;

        // Create new proposal
        let proposal = Proposal {
            id: proposal_count,
            proposer: proposer.clone(),
            description,
            function,
            target,
            parameters,
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Active,
            deadline,
        };

        // Store the proposal
        let mut proposals: Map<u32, Proposal> =
            env.storage().instance().get(&DataKey::Proposals).unwrap();
        proposals.set(proposal_count, proposal);

        // Update storage
        env.storage()
            .instance()
            .set(&DataKey::Proposals, &proposals);
        env.storage()
            .instance()
            .set(&DataKey::ProposalCount, &proposal_count);

        proposal_count
    }

    // Vote on a proposal
    pub fn vote(env: Env, voter: Address, proposal_id: u32, vote_for: bool) {
        voter.require_auth();

        // Check if voter is a member
        if !env.storage().instance().has(&DataKey::Members) {
            panic!("Only members can vote");
        }

        let mut members: Map<Address, Member> =
            env.storage().instance().get(&DataKey::Members).unwrap();

        // Get member details
        let mut member = members.get(voter.clone()).unwrap();

        // Check if member has already voted for this proposal
        // change it to a mapping, create an additional datakey with address (map to proposal id and bool)
        for voted_id in member.voted_proposals.iter() {
            if voted_id == proposal_id {
                panic!("Member has already voted on this proposal");
            }
        }

        // Get proposal
        let mut proposals: Map<u32, Proposal> =
            env.storage().instance().get(&DataKey::Proposals).unwrap();

        let mut proposal = proposals.get(proposal_id).unwrap();
        // Check if deadline has passed
        let current_time = env.ledger().timestamp();
        if current_time > proposal.deadline {
            panic!("Proposal deadline has passed");
        }

        // Add vote
        // create helper func to calc voting power
        // get bal of user and getting total voting power in DAO (datakey)
        // no floats -- add precision -- use powers
        let token_balance = member.token_balance;
        if vote_for {
            proposal.votes_for += token_balance;
        } else {
            proposal.votes_against += token_balance;
        }

        // Update member's voted proposals
        member.voted_proposals.push_back(proposal_id);

        // Updated storage
        members.set(voter, member);
        proposals.set(proposal_id, proposal);
        env.storage().instance().set(&DataKey::Members, &members);
        env.storage()
            .instance()
            .set(&DataKey::Proposals, &proposals);
    }

    // add a means for a user to add themselves tokens in the dao

    // Execute a proposal if it has enough votes
    pub fn execute_proposal(env: Env, caller: Address, proposal_id: u32) {
        caller.require_auth();

        // Get proposal
        let mut proposals: Map<u32, Proposal> =
            env.storage().instance().get(&DataKey::Proposals).unwrap();
        if !env.storage().instance().has(&proposal_id) {
            panic!("Proposal does not exist");
        }

        let mut proposal = proposals.get(proposal_id).unwrap();

        // Check if proposal is still active
        let current_time = env.ledger().timestamp();
        if current_time > proposal.deadline {
            panic!("Proposal deadline has passed");
        }

        // Determine if the proposal passes (simple majority)
        if proposal.votes_for > proposal.votes_against {
            // Execute the proposal by calling the specified function
            let target_client: Symbol = env.invoke_contract(
                &proposal.target,
                &proposal.function,
                proposal.parameters.clone(),
            );

            // Update proposal status
            proposal.status = ProposalStatus::Executed;
        } else {
            // Update proposal status to rejected
            proposal.status = ProposalStatus::Rejected;
        }

        // Update storage
        proposals.set(proposal_id, proposal);
        env.storage()
            .instance()
            .set(&DataKey::Proposals, &proposals);
    }
    // View functions
    pub fn get_member(env: Env, address: Address) -> Option<Member> {
        let members: Map<Address, Member> =
            env.storage().instance().get(&DataKey::Members).unwrap();
        if env.storage().instance().has(&address) {
            Some(members.get(address).unwrap())
        } else {
            None
        }
    }

    pub fn get_proposal(env: Env, proposal_id: u32) -> Option<Proposal> {
        let proposals: Map<u32, Proposal> =
            env.storage().instance().get(&DataKey::Proposals).unwrap();
        if env.storage().instance().has(&proposal_id) {
            Some(proposals.get(proposal_id).unwrap())
        } else {
            None
        }
    }
}

mod test;
