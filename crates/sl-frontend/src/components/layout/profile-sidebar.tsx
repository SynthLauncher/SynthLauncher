"use client"

import type React from "react"
import { useEffect, useState } from "react"
import type { PlayerProfile, PlayerProfiles } from "@/lib/types/profiles"
import {
  User,
  LogOut,
  Crown,
  Shield,
  ChevronRight,
  Edit3,
  Plus,
  ComputerIcon as Microsoft,
  ArrowLeft,
} from "lucide-react"
import { Button } from "@/components/ui/button"
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
import { createOfflineAccount, getAllProfiles, setCurrentProfile } from "@/lib/commands/profiles"

export const ProfileSidebar: React.FC = () => {
  const [isLoading, setIsLoading] = useState(true)
  const [isAddAccountOpen, setIsAddAccountOpen] = useState(false)
  const [isOfflineFormOpen, setIsOfflineFormOpen] = useState(false)
  const [isMicrosoftAuthOpen, setIsMicrosoftAuthOpen] = useState(false)
  const [offlineUsername, setOfflineUsername] = useState("")
  const [profiles, setProfiles] = useState<PlayerProfiles>()
  const [profile, setProfile] = useState<PlayerProfile>()

  const refreshProfiles = async () => {
    setIsLoading(true)
    try {
      const prfls = await getAllProfiles()
      setProfiles(prfls)
      setProfile(prfls?.profiles[prfls.current_profile_index])
    } finally {
      setIsLoading(false)
    }
  }

  useEffect(() => {
    refreshProfiles()
  }, [])

  const handleOfflineAccountCreate = () => {
    createOfflineAccount(offlineUsername);
    setIsOfflineFormOpen(false);
    refreshProfiles();
  }

  const handleMicrosoftAuth = () => {
    setIsMicrosoftAuthOpen(false)
    setIsAddAccountOpen(false)

    refreshProfiles()
  }

  if (isLoading) {
    return (
      <div className="min-w-80 w-80 bg-neutral-800 border-l border-neutral-700 p-6 overflow-y-auto flex items-center justify-center">
        <div className="text-neutral-400">Loading profile...</div>
      </div>
    )
  }

  return (
    <div className="w-80 shrink-0 bg-layout border-l-2 border-layout overflow-y-auto flex flex-col">
      <div className="p-6 border-b-2 border-neutral-700">
        <div className="flex flex-col items-center">
          <div className="relative group mb-4">
            <div className="w-20 h-20 bg-neutral-700 rounded-2xl flex items-center justify-center border border-neutral-600 group-hover:border-blue-400/50 transition-all duration-300">
              {profile?.data.name ? (
                <img
                  src={`https://mc-heads.net/avatar/${profile.data.name}/80`}
                  alt={profile.data.name}
                  className="w-16 h-16 rounded-xl"
                  onError={(e) => {
                    e.currentTarget.style.display = "none"
                    e.currentTarget.nextElementSibling?.classList.remove("hidden")
                  }}
                />
              ) : null}
              <User className="w-10 h-10 text-blue-400 hidden" />
            </div>
            <div className="absolute -bottom-1 -right-1 w-6 h-6 bg-neutral-700 rounded-full flex items-center justify-center border border-neutral-600">
              {profile?.premium ? (
                <Crown className="w-3 h-3 text-yellow-400" />
              ) : (
                <Shield className="w-3 h-3 text-neutral-400" />
              )}
            </div>
            <div className="absolute inset-0 w-20 h-20 bg-blue-400/10 rounded-2xl blur-md opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
          </div>

          <div className="text-center space-y-2 w-full">
            <div className="flex items-center justify-center gap-2">
              <h2 className="text-xl font-bold text-white text-center">{profile?.data.name || "Unknown Player"}</h2>
            </div>

            <div className="space-y-1">
              <div className="text-xs text-neutral-400 text-center">ID: {profile?.data.id || "N/A"}</div>
              <div className="flex items-center justify-center gap-2">
                <div
                  className={`inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium ${
                    profile?.premium
                      ? "bg-yellow-500/20 text-yellow-400 border border-yellow-500/30"
                      : "bg-neutral-700 text-neutral-400 border border-neutral-600"
                  }`}
                >
                  {profile?.premium ? (
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

      <div className="mt-auto p-6">
        <div className="space-y-2">
          <DropdownMenu>
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
              <div className="max-h-72 overflow-y-auto">
                {profiles?.profiles.map((account, idx) => {
                  const isCurrent = idx === profiles.current_profile_index
                  return (
                    <DropdownMenuItem
                      key={account.data.id}
                      onClick={async () => {
                        setCurrentProfile(idx)
                        await refreshProfiles()
                      }}
                      className="flex items-center gap-3 p-3 text-neutral-300 hover:text-white hover:bg-neutral-700 focus:bg-neutral-700 focus:text-white"
                    >
                      <img
                        src={`https://mc-heads.net/avatar/${account.data.name}/80`}
                        alt={account.data.id}
                        className="w-8 h-8 rounded-lg"
                      />
                      <div className="flex-1">
                        <div className="flex items-center gap-2">
                          <span className="font-medium">{account.data.name}</span>
                          {isCurrent && <div className="w-2 h-2 bg-green-400 rounded-full" />}
                        </div>
                        <div className="text-xs text-neutral-400">
                          {account.premium ? "Premium Account" : "Offline Account"}
                        </div>
                      </div>
                      {account.premium && <Crown className="w-4 h-4 text-yellow-400" />}
                    </DropdownMenuItem>
                  )
                })}
              </div>
              <DropdownMenuSeparator className="bg-neutral-700" />
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

          <Button
            variant="ghost"
            className="w-full justify-start text-red-400 hover:text-red-300 hover:bg-red-500/10 transition-all duration-300 group"
          >
            <LogOut className="w-4 h-4 mr-3" />
            Sign Out
            <ChevronRight className="w-4 h-4 ml-auto opacity-0 group-hover:opacity-100 transition-opacity" />
          </Button>
        </div>

        <div className="mt-6 pt-4 border-t-2 border-neutral-700 text-center">
          <p className="text-sm font-semibold text-sky-400">SynthLauncher v0.0.1</p>
        </div>
      </div>

      {/* Add Account Selection Dialog */}
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
                setIsMicrosoftAuthOpen(true)
              }}
            >
              <Microsoft className="w-6 h-6" />
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

      {/* Offline Account Creation Dialog */}
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
                onClick={() => setIsOfflineFormOpen(false)}
                className="flex-1 border-neutral-600 bg-neutral-700/50 hover:bg-neutral-700 text-neutral-300 hover:text-white"
              >
                Cancel
              </Button>
              <Button
                onClick={handleOfflineAccountCreate}
                disabled={!offlineUsername.trim() || offlineUsername.length < 3}
                className="flex-1 bg-blue-600 hover:bg-blue-500 text-white disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Create Account
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* Microsoft Authentication Dialog */}
      <Dialog open={isMicrosoftAuthOpen} onOpenChange={setIsMicrosoftAuthOpen}>
        <DialogContent className="bg-neutral-800 border-neutral-700 text-white">
          <DialogHeader>
            <div className="flex items-center gap-3">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  setIsMicrosoftAuthOpen(false)
                  setIsAddAccountOpen(true)
                }}
                className="text-neutral-400 hover:text-white p-1"
              >
                <ArrowLeft className="w-4 h-4" />
              </Button>
              <div>
                <DialogTitle className="text-xl font-bold">Microsoft Account</DialogTitle>
                <DialogDescription className="text-neutral-400">
                  Sign in with your Microsoft account to access premium features
                </DialogDescription>
              </div>
            </div>
          </DialogHeader>
          <div className="space-y-4 mt-6">
            <div className="text-center py-8">
              <Microsoft className="w-16 h-16 text-blue-400 mx-auto mb-4" />
              <h3 className="text-lg font-semibold text-white mb-2">Ready to Sign In</h3>
              <p className="text-neutral-400 mb-6">
                Click the button below to open Microsoft's authentication page in your browser.
              </p>
              <Button
                onClick={handleMicrosoftAuth}
                className="bg-blue-600 hover:bg-blue-500 text-white px-8 py-3 text-lg"
              >
                <Microsoft className="w-5 h-5 mr-2" />
                Sign in with Microsoft
              </Button>
            </div>
            <div className="flex justify-end pt-4 border-t border-neutral-700">
              <Button
                variant="outline"
                onClick={() => setIsMicrosoftAuthOpen(false)}
                className="border-neutral-600 bg-neutral-700/50 hover:bg-neutral-700 text-neutral-300 hover:text-white"
              >
                Cancel
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  )
}
