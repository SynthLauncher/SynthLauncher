import { useEffect, useState } from "react"
import { CurseforgeStoreCard, ModrinthStoreCard } from "@/components/layout/pages/store/store-card"
import { Input } from "@/components/ui/input"
import {
  SearchIcon,
  WifiOff,
  Loader2,
  AlertCircle,
  ChevronFirst,
  ChevronLeft,
  ChevronRight,
  ChevronLast,
  Globe,
  Blocks,
  Download,
  ArrowLeft,
  Package,
} from "lucide-react"
import { StoreCategorySelector } from "@/components/layout/pages/store/store-category-selector"
import { getCurseforgeStoreSearch, getModrinthStoreSearch } from "@/lib/commands/store"
import { getInstances } from "@/lib/commands/instances"
import type { CurseforgeSearchResult } from "@/lib/types/store/curseforge"
import type { ModrinthSearchResult } from "@/lib/types/store/modrinth"
import type { Instance } from "@/lib/types/instances"
import { Pagination, PaginationContent, PaginationItem, PaginationLink } from "@/components/ui/pagination"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"

export const StorePage = () => {
  const [store, setStore] = useState<"modrinth" | "curseforge">("modrinth")
  const [modrinthResult, setModrinthResult] = useState<ModrinthSearchResult>()
  const [curseforgeResult, setCurseforgeResult] = useState<CurseforgeSearchResult>()
  const [searchQuery, setSearchQuery] = useState("")
  const [category, setCategory] = useState("modpack")
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [isOnline, setIsOnline] = useState(true)
  const [page, setPage] = useState(1)
  const [instances, setInstances] = useState<Instance[]>([])
  const [selectedContent, setSelectedContent] = useState<any>(null)
  const [selectedVersion, setSelectedVersion] = useState<any>(null)
  const [isVersionDialogOpen, setIsVersionDialogOpen] = useState(false)
  const [isInstanceDialogOpen, setIsInstanceDialogOpen] = useState(false)

  const mockVersions = [
    {
      id: "v1.2.3",
      name: "1.2.3",
      version_number: "1.2.3",
      game_versions: ["1.20.1", "1.20.2"],
      loaders: ["fabric", "forge"],
      date_published: "2024-01-15T10:30:00Z",
      downloads: 15420,
      featured: true,
    },
    {
      id: "v1.2.2",
      name: "1.2.2",
      version_number: "1.2.2",
      game_versions: ["1.20.1"],
      loaders: ["fabric"],
      date_published: "2024-01-10T14:20:00Z",
      downloads: 12350,
      featured: false,
    },
    {
      id: "v1.2.1",
      name: "1.2.1",
      version_number: "1.2.1",
      game_versions: ["1.19.4", "1.20.1"],
      loaders: ["fabric", "forge"],
      date_published: "2023-12-20T09:15:00Z",
      downloads: 8920,
      featured: false,
    },
  ]

  const CURSEFORGE_CLASS_IDS: Record<string, number> = {
    modpack: 4471,
    mod: 6,
    resourcepack: 12,
    shader: 6552,
  }

  useEffect(() => {
    const checkOnline = () => setIsOnline(navigator.onLine)
    window.addEventListener("online", checkOnline)
    window.addEventListener("offline", checkOnline)
    checkOnline()
    return () => {
      window.removeEventListener("online", checkOnline)
      window.removeEventListener("offline", checkOnline)
    }
  }, [])

  useEffect(() => {
    const fetchInstances = async () => {
      const instanceList = await getInstances()
      setInstances(instanceList || [])
    }
    fetchInstances()
  }, [])

  useEffect(() => {
    const fetch = async () => {
      if (!isOnline) {
        setError("offline")
        return
      }
      setLoading(true)
      setError(null)
      try {
        if (store === "curseforge") {
          const classId = CURSEFORGE_CLASS_IDS[category] ?? 4471
          const result = await getCurseforgeStoreSearch(searchQuery, classId, page)
          setCurseforgeResult(result)
        } else {
          const result = await getModrinthStoreSearch(searchQuery, category, page)
          setModrinthResult(result)
        }
      } catch (err) {
        setError("failed")
      } finally {
        setLoading(false)
      }
    }
    fetch()
  }, [searchQuery, category, isOnline, store, page])

  const lastPage =
    store === "modrinth" ? (modrinthResult?.total_hits ? Math.ceil(modrinthResult.total_hits / 16) : null) : 625

  const totalResults = store === "modrinth" ? modrinthResult?.total_hits : curseforgeResult?.pagination?.totalCount

  const handleContentClick = (content: any) => {
    if (category === "modpack") {
      console.log("Installing modpack:", content)
    } else {
      setSelectedContent(content)
      setIsVersionDialogOpen(true)
    }
  }

  const handleVersionSelect = (version: any) => {
    setSelectedVersion(version)
    setIsVersionDialogOpen(false)
    setIsInstanceDialogOpen(true)
  }

  const handleInstallToInstance = (instance: Instance) => {
    setIsInstanceDialogOpen(false)
    setIsVersionDialogOpen(false)
    setSelectedContent(null)
    setSelectedVersion(null)
  }

  return (
    <div className="flex flex-col gap-3">
      <div className="flex flex-col gap-4 mb-2">
        <div className="flex flex-col sm:flex-row items-start sm:items-center gap-4 sm:gap-6">
          <div className="flex items-center gap-4">
            <StoreCategorySelector
              values={["modrinth", "curseforge"]}
              displayValues={["Modrinth", "Curseforge"]}
              defaultValue="modrinth"
              onValueChange={(v) => {
                setPage(1)
                setStore(v.toLowerCase() as "modrinth" | "curseforge")
              }}
            />
            <div className="w-[2px] h-6 bg-neutral-600" />
            <StoreCategorySelector
              values={["modpack", "mod", "shader", "resourcepack"]}
              displayValues={["Modpacks", "Mods", "Shaders", "Resource Packs"]}
              defaultValue="modpack"
              onValueChange={(value: string) => {
                setPage(1)
                setCategory(value.toLowerCase())
              }}
            />
          </div>
          {!loading && !error && totalResults && (
            <div className="flex items-center gap-2 text-sm text-neutral-400 ml-auto">
              <Globe className="w-4 h-4" />
              <span>
                <span className="text-white font-medium">{totalResults.toLocaleString()}</span> results from{" "}
                <span className="text-blue-400 font-medium capitalize">{store}</span>
              </span>
            </div>
          )}
        </div>
        <div className="relative w-full">
          <Input
            icon={<SearchIcon className="w-4 h-4 text-neutral-400" />}
            placeholder={`Search ${category}s...`}
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="bg-neutral-700/50 border-neutral-700 text-white placeholder-neutral-400 focus:border-sky-400/50 focus:bg-neutral-800/70 transition-all duration-300 h-12 text-base"
          />
          {searchQuery && (
            <div className="absolute right-3 top-1/2 -translate-y-1/2">
              <button
                onClick={() => setSearchQuery("")}
                className="w-6 h-6 rounded-full bg-neutral-600 hover:bg-neutral-500 flex items-center justify-center text-neutral-300 hover:text-white transition-colors"
              >
                ×
              </button>
            </div>
          )}
        </div>
      </div>

      {loading && (
        <div className="flex flex-col items-center justify-center py-16 px-6">
          <div className="relative mb-4">
            <Loader2 className="animate-spin w-8 h-8 text-blue-400" />
            <div className="absolute inset-0 w-8 h-8 border-2 border-blue-400/20 rounded-full animate-pulse"></div>
          </div>
          <h3 className="text-lg font-semibold text-white mb-2">Loading Content</h3>
          <p className="text-neutral-400 text-center">
            Searching {store} for the best {category}s...
          </p>
        </div>
      )}

      {error === "offline" && (
        <div className="flex flex-col items-center justify-center py-16 px-6">
          <div className="w-20 h-20 bg-neutral-700/50 rounded-full flex items-center justify-center mb-6">
            <WifiOff className="w-10 h-10 text-neutral-400" />
          </div>
          <h3 className="text-xl font-semibold text-white mb-3">You're Offline</h3>
          <p className="text-neutral-400 text-center max-w-md mb-4">
            Please check your internet connection to browse and download content from the store.
          </p>
          <div className="bg-neutral-700/50 rounded-lg p-4 border border-neutral-600">
            <p className="text-sm text-neutral-400">
              💡 Tip: You can still manage your existing instances while offline
            </p>
          </div>
        </div>
      )}

      {error === "failed" && (
        <div className="flex flex-col items-center justify-center py-16 px-6">
          <div className="w-20 h-20 bg-red-500/10 rounded-full flex items-center justify-center mb-6">
            <AlertCircle className="w-10 h-10 text-red-400" />
          </div>
          <h3 className="text-xl font-semibold text-red-400 mb-3">Failed to Load Content</h3>
          <p className="text-neutral-400 text-center max-w-md mb-4">
            We couldn't connect to the {store} servers. Please try again later.
          </p>
        </div>
      )}

      {!loading &&
        !error &&
        store === "curseforge" &&
        curseforgeResult?.data.map((hit) => (
          <div key={hit.id} onClick={() => handleContentClick(hit)}>
            <CurseforgeStoreCard hit={hit} />
          </div>
        ))}

      {!loading &&
        !error &&
        store === "modrinth" &&
        modrinthResult?.hits.map((hit) => (
          <div key={hit.project_id} onClick={() => handleContentClick(hit)}>
            <ModrinthStoreCard hit={hit} />
          </div>
        ))}

      {!loading && !error && lastPage && lastPage > 1 && (
        <div className="border-t border-[#3a3d4f]/50 p-6">
          <Pagination>
            <PaginationContent className="gap-2">
              {page > 1 && (
                <>
                  <PaginationItem>
                    <PaginationLink
                      onClick={() => setPage(1)}
                      aria-label="Go to first page"
                      size="icon"
                      className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                    >
                      <ChevronFirst className="h-4 w-4" />
                    </PaginationLink>
                  </PaginationItem>
                  <PaginationItem>
                    <PaginationLink
                      onClick={() => setPage(page - 1)}
                      aria-label="Go to previous page"
                      size="icon"
                      className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                    >
                      <ChevronLeft className="h-4 w-4" />
                    </PaginationLink>
                  </PaginationItem>
                </>
              )}
              <PaginationItem>
                <PaginationLink
                  onClick={() => setPage(page)}
                  isActive
                  className="bg-sky-600 border-sky-500 text-white hover:bg-sky-500"
                >
                  {page}
                </PaginationLink>
              </PaginationItem>
              {lastPage && page + 1 <= lastPage && (
                <PaginationItem>
                  <PaginationLink
                    onClick={() => setPage(page + 1)}
                    className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                  >
                    {page + 1}
                  </PaginationLink>
                </PaginationItem>
              )}
              {lastPage && page + 2 <= lastPage && (
                <PaginationItem>
                  <PaginationLink
                    onClick={() => setPage(page + 2)}
                    className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                  >
                    {page + 2}
                  </PaginationLink>
                </PaginationItem>
              )}
              {lastPage && page < lastPage && (
                <>
                  <PaginationItem>
                    <PaginationLink
                      onClick={() => setPage(page + 1)}
                      aria-label="Go to next page"
                      size="icon"
                      className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                    >
                      <ChevronRight className="h-4 w-4" />
                    </PaginationLink>
                  </PaginationItem>
                  <PaginationItem>
                    <PaginationLink
                      onClick={() => setPage(lastPage)}
                      aria-label="Go to last page"
                      size="icon"
                      className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                    >
                      <ChevronLast className="h-4 w-4" />
                    </PaginationLink>
                  </PaginationItem>
                </>
              )}
            </PaginationContent>
          </Pagination>
          <div className="flex items-center justify-center mt-4 text-sm text-neutral-400">
            <span>
              Page <span className="text-white font-medium">{page}</span> of{" "}
              <span className="text-white font-medium">{lastPage}</span>
            </span>
          </div>
        </div>
      )}

      {/* Version Selection Dialog */}
      <Dialog open={isVersionDialogOpen} onOpenChange={setIsVersionDialogOpen}>
        <DialogContent className="bg-neutral-800 border-neutral-700 text-white max-w-4xl max-h-[80vh] overflow-hidden">
          <DialogHeader className="border-b border-neutral-700 pb-4">
            <DialogTitle className="text-xl font-bold text-white flex items-center gap-3">
              <Package className="w-5 h-5 text-sky-400" />
              Choose Version - {selectedContent?.title || selectedContent?.name}
            </DialogTitle>
            <DialogDescription className="text-neutral-400">
              Select which version of this {category} you want to install
            </DialogDescription>
          </DialogHeader>

          <div className="py-4">
            <div className="space-y-3 max-h-96 overflow-y-auto">
              {mockVersions.map((version) => (
                <div
                  key={version.id}
                  className="bg-[#1D2026] hover:bg-[#1D2026]/90 rounded-lg p-5 flex gap-4 max-w-full transition-colors duration-200 border border-transparent hover:border-neutral-700/50 cursor-pointer group"
                  onClick={() => handleVersionSelect(version)}
                >
                  <div className="flex flex-col grow justify-between min-w-0">
                      <h1 className="text-white text-lg font-bold leading-tight group-hover:text-sky-400 transition-colors">
                        {version.name}
                      </h1>

                    <div className="flex gap-2 mt-3">
                      {version.game_versions.map((gameVersion) => (
                        <span
                          key={gameVersion}
                          className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-500/20 text-blue-400 border border-blue-500/30"
                        >
                          {gameVersion}
                        </span>
                      ))}
                      {version.loaders.map((loader) => (
                        <span
                          key={loader}
                          className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-emerald-500/20 text-emerald-400 border border-emerald-500/30"
                        >
                          {loader}
                        </span>
                      ))}
                    </div>
                  </div>

                  <div className="flex flex-col justify-between items-end ml-4 flex-shrink-0">
                    <Button
                      variant="install"
                      className="gap-2 hover:scale-105 transition-transform duration-200"
                      onClick={(e) => {
                        e.stopPropagation()
                        handleVersionSelect(version)
                      }}
                    >
                      Select
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="flex justify-end gap-3 pt-4 border-t border-neutral-700">
            <Button
              variant="outline"
              onClick={() => setIsVersionDialogOpen(false)}
              className="border-neutral-600 bg-neutral-700/50 hover:bg-neutral-700 text-neutral-300 hover:text-white"
            >
              Cancel
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      <Dialog open={isInstanceDialogOpen} onOpenChange={setIsInstanceDialogOpen}>
        <DialogContent className="bg-neutral-800 border-neutral-700 text-white max-w-4xl max-h-[80vh] overflow-hidden">
          <DialogHeader className="border-b border-neutral-700 pb-4">
            <div className="flex items-center gap-3">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  setIsInstanceDialogOpen(false)
                  setIsVersionDialogOpen(true)
                }}
                className="text-neutral-400 hover:text-white p-1"
              >
                <ArrowLeft className="w-4 h-4" />
              </Button>
              <div>
                <DialogTitle className="text-xl font-bold text-white flex items-center gap-3">
                  <Download className="w-5 h-5 text-sky-400" />
                  Install to Instance
                </DialogTitle>
                <DialogDescription className="text-neutral-400">
                  Installing {selectedContent?.title || selectedContent?.name} v{selectedVersion?.name}
                </DialogDescription>
              </div>
            </div>
          </DialogHeader>

          <div className="py-4">
            {instances.length > 0 ? (
              <div className="space-y-3 max-h-96 overflow-y-auto">
                {instances.map((instance) => (
                  <div
                    key={instance.name}
                    className="bg-[#1D2026] hover:bg-[#1D2026]/90 rounded-lg p-5 flex gap-4 max-w-full transition-colors duration-200 border border-transparent hover:border-neutral-700/50 cursor-pointer group"
                    onClick={() => handleInstallToInstance(instance)}
                  >
                    <div className="flex-shrink-0">
                      <div className="w-16 h-16 rounded-md bg-neutral-700 flex items-center justify-center">
                        <Blocks className="w-8 h-8 text-sky-400" />
                      </div>
                    </div>

                    <div className="flex flex-col grow justify-between min-w-0">
                      <div>
                        <h1 className="text-white text-lg font-bold leading-tight group-hover:text-sky-400 transition-colors">
                          {instance.name}
                          <span className="text-gray-500 font-normal text-sm ml-2">
                            {instance.mod_loader} • {instance.game_metadata.id}
                          </span>
                        </h1>
                      </div>

                      <div className="flex gap-4 text-gray-400 text-sm mt-3">
                        <div className="flex items-center gap-1">
                          <Blocks className="w-4 h-4" />
                          <span className="font-medium">Instance</span>
                        </div>
                      </div>
                    </div>

                    <div className="flex flex-col justify-between items-end ml-4 flex-shrink-0">
                      <Button
                        variant="install"
                        className="gap-2 hover:scale-105 transition-transform duration-200"
                        onClick={(e) => {
                          e.stopPropagation()
                          handleInstallToInstance(instance)
                        }}
                      >
                        <Download className="w-4 h-4" />
                        Install
                      </Button>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="flex flex-col items-center justify-center py-16 px-6">
                <div className="w-20 h-20 bg-neutral-700/50 rounded-full flex items-center justify-center mb-6">
                  <Blocks className="w-10 h-10 text-neutral-400" />
                </div>
                <h3 className="text-xl font-semibold text-white mb-3">No Instances Found</h3>
                <p className="text-neutral-400 text-center max-w-md mb-4">
                  Create an instance first to install {category}s
                </p>
                <Button className="bg-sky-600 hover:bg-sky-500 text-white px-6 py-2 rounded-lg font-medium transition-all duration-300">
                  Create Instance
                </Button>
              </div>
            )}
          </div>

          <div className="flex justify-end gap-3 pt-4 border-t border-neutral-700">
            <Button
              variant="outline"
              onClick={() => setIsInstanceDialogOpen(false)}
              className="border-neutral-600 bg-neutral-700/50 hover:bg-neutral-700 text-neutral-300 hover:text-white"
            >
              Cancel
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  )
}
