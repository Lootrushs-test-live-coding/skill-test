export type ReleaseStatus = "funded" | "released" | "refunded";
export type Role = "buyer" | "seller" | "arbiter";

export type ReleasePanelProps = {
  seller: string;
  amount: string;
  status: ReleaseStatus;
  role: Role;
  onRelease: () => Promise<void>;
};

export function ReleasePanel(_props: ReleasePanelProps) {
  // Implement this function.
  return null;
}
