import { useEffect, useState, useRef } from "react"
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogOverlay } from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { getMinecraftVersions } from "@/lib/commands/minecraft"
import { ArrowUpNarrowWide, Box, Plus, Upload, X, ChevronDown, Search, Loader2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"

export const CreateInstanceDialog = ({
  open,
  onOpenChange,
  onCreate,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
  onCreate: (name: string, version: string, loader: string) => void
}) => {
  const [name, setName] = useState("")
  const [version, setVersion] = useState("")
  const [loader, setLoader] = useState("Vanilla")
  const [iconFile, setIconFile] = useState<File | null>(null)
  const [iconPreview, setIconPreview] = useState<string | null>(null)
  const [versionFilter, setVersionFilter] = useState("")
  const [isVersionDropdownOpen, setIsVersionDropdownOpen] = useState(false)
  const [isLoaderDropdownOpen, setIsLoaderDropdownOpen] = useState(false)
  const [showAdvanced, setShowAdvanced] = useState(false)
  const [isLoadingVersions, setIsLoadingVersions] = useState(false)

  const minecraftVersionsRef = useRef<string[]>([])

  const loaderOptions = [
    { value: "Vanilla", label: "Vanilla" },
    { value: "Fabric", label: "Fabric" },
    { value: "Forge", label: "Forge" },
    { value: "NeoForge", label: "NeoForge" },
    { value: "Quilt", label: "Quilt" },
  ]

  const filteredVersions = minecraftVersionsRef.current.filter((v) => v.toLowerCase().includes(versionFilter.toLowerCase()))

  useEffect(() => {
    const fetchMinecraftVersions = async () => {
      if (minecraftVersionsRef.current.length > 0) {
        return; // If versions are already fetched, don't fetch again
      }

      setIsLoadingVersions(true)
      try {
        const versions = await getMinecraftVersions() as string[]
        minecraftVersionsRef.current = versions
        setVersion(versions[0])
      } catch (error) {
        console.error("Failed to fetch Minecraft versions:", error)
      } finally {
        setIsLoadingVersions(false)
      }
    }

    if (open) {
      fetchMinecraftVersions()
    }
  }, [open])

  const handleIconUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file && file.type.startsWith("image/")) {
      setIconFile(file)
      const reader = new FileReader()
      reader.onload = (e) => {
        setIconPreview(e.target?.result as string)
      }
      reader.readAsDataURL(file)
    }
  }

  const handleRemoveIcon = () => {
    setIconFile(null)
    setIconPreview(null)
  }

  const handleCreate = () => {
    if (!name.trim()) {
      return
    }
    onOpenChange(false)
    onCreate(name, version, loader)

    setName("")
    setVersion("")
    setLoader("Vanilla")
    setIconFile(null)
    setIconPreview(null)
    setShowAdvanced(false)
    setVersionFilter("")
  }

  const handleCancel = () => {
    onOpenChange(false)

    setName("")
    setVersion("")
    setLoader("Vanilla")
    setIconFile(null)
    setIconPreview(null)
    setShowAdvanced(false)
    setVersionFilter("")
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogOverlay className="bg-black/60 backdrop-blur-sm" />
      <DialogContent className="sm:max-w-[500px] bg-[#1E2128] border-[#2c3039] border-2 shadow-2xl">
        <DialogHeader>
          <DialogTitle className="text-white text-xl font-semibold">Create New Instance</DialogTitle>
          <DialogDescription className="text-gray-300">
            Configure your new Minecraft instance with custom settings.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-6 py-4">
          {/* Instance Name */}
          <div className="grid gap-2">
            <Label htmlFor="name" className="text-sm font-medium text-gray-200">
              Instance Name *
            </Label>
            <Input
              id="name"
              className="border-0 bg-[#2b3136] text-white placeholder:text-[#9ca5a8] focus:ring-2 focus:ring-[#f56241]/50 transition-all"
              placeholder="My awesome instance"
              value={name}
              onChange={(e) => setName(e.target.value)}
              maxLength={50}
            />
            {name.length > 40 && <p className="text-xs text-yellow-400">{50 - name.length} characters remaining</p>}
          </div>

          {/* Minecraft Version */}
          <div className="grid gap-2">
            <Label htmlFor="version" className="text-sm font-medium text-gray-200">
              Minecraft Version
            </Label>
            <div className="relative">
              <button
                type="button"
                onClick={() => setIsVersionDropdownOpen(!isVersionDropdownOpen)}
                className="appearance-none flex h-10 w-full rounded-md bg-[#2b3136] px-3 py-2 text-sm text-gray-200 justify-between items-center outline-none hover:bg-[#323842] transition-colors focus:ring-2 focus:ring-[#f56241]/50"
                disabled={isLoadingVersions}
              >
                <span>{version || "Select version..."}</span>
                {isLoadingVersions ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <ChevronDown
                    className={`w-4 h-4 transition-transform ${isVersionDropdownOpen ? "rotate-180" : ""}`}
                  />
                )}
              </button>

              {isVersionDropdownOpen && !isLoadingVersions && (
                <div className="absolute z-50 w-full mt-1 bg-[#2b3136] border border-[#2c3039] rounded-md shadow-lg max-h-60 overflow-hidden">
                  <div className="p-2 border-b border-[#2c3039]">
                    <div className="relative">
                      <Search className="absolute left-2 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
                      <Input
                        placeholder="Search versions..."
                        value={versionFilter}
                        onChange={(e) => setVersionFilter(e.target.value)}
                        className="pl-8 bg-[#1E2128] border-0 text-white placeholder:text-gray-400 h-8"
                      />
                    </div>
                  </div>
                  <div className="max-h-48 overflow-y-auto">
                    {filteredVersions.map((v) => (
                      <button
                        key={v}
                        onClick={() => {
                          setVersion(v)
                          setIsVersionDropdownOpen(false)
                          setVersionFilter("")
                        }}
                        className="w-full px-3 py-2 text-left text-sm text-gray-200 hover:bg-[#323842] transition-colors"
                      >
                        {v}
                      </button>
                    ))}
                    {filteredVersions.length === 0 && (
                      <div className="px-3 py-2 text-sm text-gray-400">No versions found</div>
                    )}
                  </div>
                </div>
              )}
            </div>
          </div>

          {/* Mod Loader */}
          <div className="grid gap-2">
            <Label htmlFor="modloader" className="text-sm font-medium text-gray-200">
              Mod Loader
            </Label>
            <div className="relative">
              <button
                type="button"
                onClick={() => setIsLoaderDropdownOpen(!isLoaderDropdownOpen)}
                className="appearance-none flex h-10 w-full rounded-md bg-[#2b3136] px-3 py-2 text-sm text-gray-200 justify-between items-center outline-none hover:bg-[#323842] transition-colors focus:ring-2 focus:ring-[#f56241]/50"
              >
                <div className="flex flex-col items-start">
                  <span>{loaderOptions.find((l) => l.value === loader)?.label || "Select loader..."}</span>
                </div>
                <ChevronDown className={`w-4 h-4 transition-transform ${isLoaderDropdownOpen ? "rotate-180" : ""}`} />
              </button>

              {isLoaderDropdownOpen && (
                <div className="absolute z-50 w-full mt-1 bg-[#2b3136] border border-[#2c3039] rounded-md shadow-lg">
                  {loaderOptions.map((option) => (
                    <button
                      key={option.value}
                      onClick={() => {
                        setLoader(option.value)
                        setIsLoaderDropdownOpen(false)
                      }}
                      className="w-full px-3 py-3 text-left hover:bg-[#323842] transition-colors border-b border-[#2c3039] last:border-b-0"
                    >
                      <div className="flex flex-col">
                        <span className="text-sm text-gray-200 font-medium">{option.label}</span>
                      </div>
                    </button>
                  ))}
                </div>
              )}
            </div>
          </div>

          {/* Instance Icon */}
          <div className="grid gap-2">
            <Label htmlFor="icon" className="text-sm font-medium text-gray-200">
              Instance Icon
            </Label>
            <div className="flex items-center gap-4">
              <div className="w-20 h-20 bg-[#2b3136] rounded-lg flex items-center justify-center border-2 border-dashed border-[#2c3039] overflow-hidden">
                {iconPreview ? (
                  <img
                    src={iconPreview || "/placeholder.svg"}
                    alt="Instance icon"
                    className="w-full h-full object-cover rounded-md"
                  />
                ) : (
                  <Box className="text-neutral-400" width={32} height={32} />
                )}
              </div>
              <div className="flex flex-col justify-center gap-2">
                <label htmlFor="icon-upload">
                  <input id="icon-upload" type="file" accept="image/*" onChange={handleIconUpload} className="hidden" />
                  <Button
                    type="button"
                    className="px-4 py-2 bg-[#2b3136] hover:bg-[#323842] text-gray-200 rounded-lg transition-colors flex gap-2 items-center cursor-pointer"
                    onClick={() => document.getElementById("icon-upload")?.click()}
                  >
                    <Upload width={16} height={16} />
                    Choose Icon
                  </Button>
                </label>
                {iconPreview && (
                  <Button
                    type="button"
                    onClick={handleRemoveIcon}
                    className="px-4 py-2 hover:bg-[#d14646] bg-[#f34b4b] text-white rounded-lg transition-colors flex gap-2 items-center"
                  >
                    <X width={16} height={16} />
                    Remove
                  </Button>
                )}
              </div>
            </div>
            <p className="text-xs text-gray-400">Recommended: 64x64 pixels, PNG or JPG format</p>
          </div>
        </div>

        <DialogFooter className="gap-2">
          <Button
            onClick={() => setShowAdvanced(!showAdvanced)}
            className="px-4 py-2 bg-[#2b3136] hover:bg-[#323842] text-gray-200 rounded-lg transition-colors flex items-center gap-2"
          >
            <ArrowUpNarrowWide width={16} height={16} />
            <span>{showAdvanced ? "Hide" : "Show"} Advanced</span>
          </Button>

          <Button
            onClick={handleCancel}
            className="px-4 py-2 bg-[#2b3136] hover:bg-[#323842] text-gray-200 rounded-lg transition-colors flex items-center gap-2"
          >
            <X width={16} height={16} />
            <span>Cancel</span>
          </Button>

          <Button
            onClick={handleCreate}
            disabled={!name.trim() || !version || !loader}
            className="px-6 py-2 bg-[#f56241] hover:bg-[#f56241]/80 disabled:bg-[#f56241]/50 disabled:cursor-not-allowed text-white rounded-lg transition-colors flex items-center gap-2"
          >
            <Plus width={16} height={16} />
            <span>Create Instance</span>
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
