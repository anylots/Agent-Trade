"use client";

import { MemeTab } from "@/components/meme-tab";
import { HeatChart } from "@/components/heat-chart";

export default function MemePage() {
  return (
    <div className="space-y-6">
      <HeatChart />
      <MemeTab />
    </div>
  );
}
