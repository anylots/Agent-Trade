"use client";

import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { AISignalsTab } from "@/components/ai-signals-tab";
import { MemeTab } from "@/components/meme-tab";
import { HeatChart } from "@/components/heat-chart";

export default function Home() {
  const [activeTab, setActiveTab] = useState("ai-signals");

  return (
    <div className="space-y-6">
      <HeatChart />

      <Tabs defaultValue="ai-signals" className="w-full" onValueChange={setActiveTab}>
        <TabsList className="mb-4">
          <TabsTrigger value="ai-signals" className="text-base">AI信号</TabsTrigger>
          <TabsTrigger value="meme" className="text-base">Meme</TabsTrigger>
        </TabsList>

        <TabsContent value="ai-signals">
          <AISignalsTab />
        </TabsContent>

        <TabsContent value="meme">
          <MemeTab />
        </TabsContent>
      </Tabs>
    </div>
  );
}
