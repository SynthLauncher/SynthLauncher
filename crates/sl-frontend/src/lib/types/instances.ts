enum VersionType {
  OldBeta,
  OldAlpha,
  Release,
  Snapshot,
};

enum InstanceType {
  Vanilla,
  Fabric,
  Quilt,
};

interface InstanceGameInfo {
  id: string;
  releaseTime: string;
  type: VersionType;
};

export interface Instance {
  name: string;
  game_info: InstanceGameInfo;
  instance_type: InstanceType;
};

export type InstanceCardProps = {
  title: string;
  version: string;
  modLoader?: string;
  modCount?: number;
  lastPlayed: string;
  image: string;
  favorite?: boolean;
};

