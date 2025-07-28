
export interface PlayerData {
    id: string,
    access_token: string
}

export interface PlayerAccounts {
    current_account: string,
    accounts: Record<string, PlayerData>
}
