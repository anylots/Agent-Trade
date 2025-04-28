"use client";

import { AISignalsTab } from "@/components/ai-signals-tab";
import { HeatChart } from "@/components/heat-chart";

export default function AISignalsPage() {
  return (
    <div className="space-y-6">
      <HeatChart />
      <AISignalsTab />
    </div>
  );
}
