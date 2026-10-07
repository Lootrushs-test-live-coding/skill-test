#pragma once

#include <cstdint>
#include <optional>
#include <unordered_map>

enum class Status { Funded, Released, Refunded };

enum class Error { Ok, NotFound, NotAuthorized, BadStatus, Overflow };

struct Escrow {
    uint64_t buyer{};
    uint64_t seller{};
    uint64_t arbiter{};
    uint64_t amount{};
    Status status{Status::Funded};
};

class Ledger {
public:
    void put_escrow(uint64_t id, const Escrow& escrow) { escrows_[id] = escrow; }

    void set_balance(uint64_t account, uint64_t amount) { balances_[account] = amount; }

    uint64_t balance(uint64_t account) const {
        auto it = balances_.find(account);
        if (it == balances_.end()) {
            return 0;
        }
        return it->second;
    }

    std::optional<Escrow> escrow(uint64_t id) const {
        auto it = escrows_.find(id);
        if (it == escrows_.end()) {
            return std::nullopt;
        }
        return it->second;
    }

    Error release(uint64_t caller, uint64_t escrow_id);

private:
    std::unordered_map<uint64_t, Escrow> escrows_;
    std::unordered_map<uint64_t, uint64_t> balances_;
};
