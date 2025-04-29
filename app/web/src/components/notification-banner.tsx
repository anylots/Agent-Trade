import { AlertCircle } from "lucide-react";
import { cn } from "@/lib/utils";

export function NotificationBanner() {
  return (
    <div className="relative overflow-hidden bg-black border-t border-b border-cyan-800/50 py-2 px-4 text-center text-sm flex items-center justify-center">
      {/* <div className="absolute inset-0 bg-gradient-to-r from-cyan-900/20 via-cyan-500/30 to-cyan-900/20 animate-gradient-x pointer-events-none" />
      <span className="text-cyan-400 font-medium relative z-10 tracking-wide">
        {`>>> Scanning SNS & On-Chain information across the entire network >>>`}
      </span> */}
    </div>
  );
}
