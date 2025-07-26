import { useEffect, useState } from "react"
import {
  User,
  LogOut,
  Crown,
  Shield,
  ChevronRight,
  Edit3,
  Plus,
  ArrowLeft,
  OctagonAlert,
} from "lucide-react"
import { Button, buttonVariants } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { FaMicrosoft } from "react-icons/fa6"
import { createOfflineAccount, getAccounts, removeAccount, setCurrentAccount } from "@/lib/commands/accounts"
import { PlayerAccounts, PlayerData } from "@/lib/types/account"
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from "../ui/alert-dialog"

const ProfileDisplay = ({ account, name }: { account: PlayerData, name: string }) => {
  const isPremium = !(account.access_token === "0");

  return (
    <div className="p-6 border-b-2 border-neutral-700">
      <div className="flex flex-col items-center">
        <div className="relative group mb-4">
          <div className="w-20 h-20 bg-neutral-700 rounded-2xl flex items-center justify-center border-2 border-neutral-600 group-hover:border-blue-400/50 transition-all duration-300">
            {name ? (
              <img
                src={`https://mc-heads.net/avatar/${name}/80`}
                alt={name}
                className="w-16 h-16 rounded-xl"
              />
            ) : null}
            <User className="w-10 h-10 text-blue-400 hidden" />
          </div>
          <div className="absolute -bottom-1 -right-1 w-6 h-6 bg-neutral-700 rounded-full flex items-center justify-center border-2 border-neutral-600">
            {isPremium ? (
              <Crown className="w-3 h-3 text-yellow-400" />
            ) : (
              <Shield className="w-3 h-3 text-neutral-400" />
            )}
          </div>
          <div className="absolute inset-0 w-20 h-20 bg-blue-400/10 rounded-2xl blur-md opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
        </div>

        <div className="text-center space-y-2 w-full">
          <div className="flex items-center justify-center gap-2">
            <h2 className="text-xl font-bold text-white text-center tracking-wide">{name || "Unknown Player"}</h2>
          </div>

          <div className="space-y-1">
            <div className="text-xs text-neutral-400 text-center">ID: {account.id || "N/A"}</div>
            <div className="flex items-center justify-center gap-2">
              <div
                className={`inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium ${isPremium
                  ? "bg-yellow-500/20 text-yellow-400 border border-yellow-500/30"
                  : "bg-neutral-700 text-neutral-400 border border-neutral-600"
                  }`}
              >
                {isPremium ? (
                  <>
                    <Crown className="w-3 h-3" />
                    Premium
                  </>
                ) : (
                  <>
                    <Shield className="w-3 h-3" />
                    Offline
                  </>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

const AddProfileDialog = ({ isAddAccountOpen, setIsAddAccountOpen, setIsOfflineFormOpen }: {
  isAddAccountOpen: boolean,
  setIsAddAccountOpen: (bool: boolean) => void,
  setIsOfflineFormOpen: (bool: boolean) => void
}) => {
  return (
    <Dialog open={isAddAccountOpen} onOpenChange={setIsAddAccountOpen}>
      <DialogContent className="bg-neutral-800 border-neutral-700 text-white">
        <DialogHeader>
          <DialogTitle className="text-xl font-bold">Add New Account</DialogTitle>
          <DialogDescription className="text-neutral-400">
            Choose how you want to add your Minecraft account
          </DialogDescription>
        </DialogHeader>
        <div className="space-y-4 mt-6">
          <Button
            className="w-full justify-start gap-3 h-16 bg-blue-600 hover:bg-blue-500 text-white"
            onClick={() => {
              setIsAddAccountOpen(false)
            }}
          >
            <FaMicrosoft className="w-6 h-6" />
            <div className="text-left">
              <div className="font-semibold">Microsoft Account</div>
              <div className="text-sm text-blue-100">Sign in with your Microsoft account</div>
            </div>
          </Button>
          <Button
            variant="outline"
            className="w-full justify-start gap-3 h-16 border-neutral-600 bg-neutral-700 hover:bg-neutral-600 text-white"
            onClick={() => {
              setIsAddAccountOpen(false)
              setIsOfflineFormOpen(true)
            }}
          >
            <User className="w-6 h-6" />
            <div className="text-left">
              <div className="font-semibold">Offline Account</div>
              <div className="text-sm text-neutral-300">Create a local offline account</div>
            </div>
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}

const OfflineFormDialog = ({ isOfflineFormOpen, setIsOfflineFormOpen, setIsAddAccountOpen, refreshProfiles }: {
  isOfflineFormOpen: boolean,
  setIsOfflineFormOpen: (bool: boolean) => void,
  setIsAddAccountOpen: (bool: boolean) => void,
  refreshProfiles: () => void
}) => {
  const [offlineUsername, setOfflineUsername] = useState("")

  const handleOfflineAccountCreate = () => {
    createOfflineAccount(offlineUsername);
    setIsOfflineFormOpen(false);
    refreshProfiles();
  }

  return (
    <Dialog open={isOfflineFormOpen} onOpenChange={setIsOfflineFormOpen}>
      <DialogContent className="bg-neutral-800 border-neutral-700 text-white">
        <DialogHeader>
          <div className="flex items-center gap-3">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => {
                setIsOfflineFormOpen(false)
                setIsAddAccountOpen(true)
              }}
              className="text-neutral-400 hover:text-white p-1"
            >
              <ArrowLeft className="w-4 h-4" />
            </Button>
            <div>
              <DialogTitle className="text-xl font-bold">Create Offline Account</DialogTitle>
              <DialogDescription className="text-neutral-400">
                Enter a username for your offline account
              </DialogDescription>
            </div>
          </div>
        </DialogHeader>
        <div className="space-y-4 mt-6">
          <div className="space-y-2">
            <Label htmlFor="username" className="text-sm font-medium text-white">
              Username
            </Label>
            <Input
              id="username"
              type="text"
              placeholder="Enter username..."
              value={offlineUsername}
              onChange={(e) => setOfflineUsername(e.target.value)}
              className="bg-neutral-700 border-neutral-600 text-white placeholder-neutral-400 focus:border-blue-400"
              maxLength={16}
            />
            <p className="text-xs text-neutral-400">
              Username must be 3-16 characters long and contain only letters, numbers, and underscores.
            </p>
          </div>
          <div className="flex gap-3 pt-4">
            <Button
              variant="outline"
              onClick={() => {
                setIsOfflineFormOpen(false)
                setTimeout(() => (document.body.style.pointerEvents = ""), 500);
              }}
              className="flex-1 border-neutral-600 bg-neutral-700/50 hover:bg-neutral-700 text-neutral-300 hover:text-white"
            >
              Cancel
            </Button>
            <Button
              onClick={() => {
                handleOfflineAccountCreate()
                setTimeout(() => (document.body.style.pointerEvents = ""), 500);
              }}
              disabled={!offlineUsername.trim() || offlineUsername.length < 3}
              className="flex-1 bg-blue-600 hover:bg-blue-500 text-white disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Create Account
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}

export const ProfileSidebar = () => {
  const [isLoading, setIsLoading] = useState(true)
  const [isAddAccountOpen, setIsAddAccountOpen] = useState(false)
  const [isOfflineFormOpen, setIsOfflineFormOpen] = useState(false)
  const [profiles, setProfiles] = useState<PlayerAccounts>()
  const [account, setAccount] = useState<[string, PlayerData]>();

  const refreshProfiles = async () => {
    setIsLoading(true)
    try {
      const prfls = await getAccounts()
      setProfiles(prfls);
      setAccount([prfls?.current_account, prfls?.accounts[prfls.current_account]] as [string, PlayerData]);
    } finally {
      setIsLoading(false)
    }
  }

  useEffect(() => {
    refreshProfiles()
  }, [])

  if (isLoading) {
    return (
      <div className="min-w-80 w-80 bg-neutral-800 border-l border-neutral-700 p-6 overflow-y-auto flex items-center justify-center">
        <div className="text-neutral-400">Loading profile...</div>
      </div>
    )
  }

  return (
    <div className="w-80 shrink-0 bg-layout border-l-2 border-layout overflow-y-auto flex flex-col">
      {account?.[1] && account?.[0] ? (
        <ProfileDisplay account={account[1]} name={account[0]} />
      ) : (
        <div className="p-6 text-center text-neutral-400">
          <div className="w-20 h-20 mx-auto mb-4 rounded-2xl bg-neutral-700 flex items-center justify-center border border-neutral-600">
            <User className="w-10 h-10 text-neutral-500" />
          </div>
          <h2 className="text-lg font-semibold">No Account Found</h2>
          <p className="text-sm text-neutral-500 mt-2">
            You don't have any accounts chosen yet.
          </p>
        </div>
      )}

      <div className="mt-auto p-6">
        <div className="space-y-2">
          <DropdownMenu modal={false}>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                className="w-full justify-start text-neutral-300 hover:text-white hover:bg-neutral-700 transition-all duration-300 group"
              >
                <Edit3 className="w-4 h-4 mr-3 group-hover:text-blue-400 transition-colors" />
                Switch Account
                <ChevronRight className="w-4 h-4 ml-auto opacity-0 group-hover:opacity-100 transition-opacity" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent
              className="w-64 bg-neutral-800 border-neutral-700"
              align="start"
            >
              {(Object.keys(profiles?.accounts ?? {}).length >= 1) && (
                <>
                  <div className="max-h-72 overflow-y-auto">
                    {Object.entries(profiles?.accounts as Record<string, PlayerData>).map(([name, account]) => {
                      const isCurrent = name === profiles?.current_account as string;
                      const isPremium = "0" === account?.access_token;
                      return (
                        <DropdownMenuItem
                          key={account.id}
                          onClick={async () => {
                            setCurrentAccount(name)
                            await refreshProfiles()
                          }}
                          className="flex items-center gap-3 p-3 text-neutral-300 hover:text-white hover:bg-neutral-700 focus:bg-neutral-700 focus:text-white"
                        >
                          <img
                            src={`https://mc-heads.net/avatar/${name}/80`}
                            alt={account.id}
                            className="w-8 h-8 rounded-lg"
                          />
                          <div className="flex-1">
                            <div className="flex items-center gap-2">
                              <span className="font-medium">{name}</span>
                              {isCurrent && <div className="w-2 h-2 bg-green-400 rounded-full" />}
                            </div>
                            <div className="text-xs text-neutral-400">
                              {!isPremium ? "Premium Account" : "Offline Account"}
                            </div>
                          </div>
                          {!isPremium && <Crown className="w-4 h-4 text-yellow-400" />}
                        </DropdownMenuItem>
                      )
                    })}
                  </div>


                  <DropdownMenuSeparator className="bg-neutral-700" />
                </>
              )}

              <DropdownMenuItem
                className="flex items-center gap-3 p-3 text-blue-400 hover:text-blue-300 hover:bg-blue-500/10 focus:bg-blue-500/10 focus:text-blue-300"
                onSelect={(e) => {
                  e.preventDefault()
                  setIsAddAccountOpen(true)
                }}
              >
                <Plus className="w-4 h-4" />
                Add Account
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>

          {account?.[1] && (
            <AlertDialog>
              <AlertDialogTrigger asChild>
                <Button
                  variant="ghost"
                  className="w-full justify-start text-red-400 hover:text-red-300 hover:bg-red-500/10 transition-all duration-300 group"
                >
                  <LogOut className="w-4 h-4 mr-3" />
                  Remove The Account
                  <ChevronRight className="w-4 h-4 ml-auto opacity-0 group-hover:opacity-100 transition-opacity" />
                </Button>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader className="items-center">
                  <AlertDialogTitle>
                    <div className="mb-2 mx-auto flex h-14 w-14 items-center justify-center rounded-full bg-destructive/10">
                      <OctagonAlert className="h-7 w-7 text-destructive" />
                    </div>
                    Are you absolutely sure?
                  </AlertDialogTitle>
                  <AlertDialogDescription className="text-[15px] text-center">
                    This action cannot be undone. This will delete your
                    account in the launcher.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter className="mt-2 sm:justify-center">
                  <AlertDialogCancel>Cancel</AlertDialogCancel>
                  <AlertDialogAction
                    className={buttonVariants({ variant: "destructive" })}
                    onClick={async () => {
                      removeAccount(account?.[0] as string)
                      await refreshProfiles()
                    }}
                  >
                    Continue
                  </AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
          )}

        </div>

        <div className="mt-6 pt-4 border-t-2 border-neutral-700 text-center">
          <p className="text-sm font-normal text-neutral-400">SynthLauncher v0.0.1</p>
        </div>
      </div>

      <AddProfileDialog
        isAddAccountOpen={isAddAccountOpen}
        setIsAddAccountOpen={setIsAddAccountOpen}
        setIsOfflineFormOpen={setIsOfflineFormOpen}
      />

      <OfflineFormDialog
        isOfflineFormOpen={isOfflineFormOpen}
        setIsOfflineFormOpen={setIsOfflineFormOpen}
        setIsAddAccountOpen={setIsAddAccountOpen}
        refreshProfiles={refreshProfiles}
      />
    </div>
  )
}
