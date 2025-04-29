"use client";

import { useState } from "react";
import { Badge } from "@/components/ui/badge";

export function HeatChart() {
  const [timeRange, setTimeRange] = useState("24h");

  // Mock data for the heat chart - using deterministic pattern instead of random
  const colors = ["bg-purple-200", "bg-green-300", "bg-purple-300", "bg-green-400", "bg-pink-300"];
  const heatData = Array(24).fill(0).map((_, index) => {
    return {
      color: colors[index % colors.length]
    };
  });

  return (
    <div className="bg-gray-900 rounded-lg p-4">
      <div className="flex justify-between items-center mb-2">
        <div className="flex items-center space-x-2">
          <span className="text-sm font-medium">{timeRange} heat</span>
          <div className="flex items-center space-x-1">
            <div className="w-2 h-2 bg-green-400 rounded-full"></div>
            <span className="text-xs text-gray-400">ordinary</span>
          </div>
          <div className="flex items-center space-x-1">
            <div className="w-2 h-2 bg-purple-400 rounded-full"></div>
            <span className="text-xs text-gray-400">hot</span>
          </div>
          <div className="flex items-center space-x-1">
            <div className="w-2 h-2 bg-pink-400 rounded-full"></div>
            <span className="text-xs text-gray-400">hot</span>
          </div>
        </div>

        <div className="flex space-x-2">
          <Badge
            variant={timeRange === "24h" ? "default" : "outline"}
            className={`cursor-pointer ${timeRange === "24h" ? "bg-green-400" : "bg-transparent text-gray-400"}`}
            onClick={() => setTimeRange("24h")}
          >
            24h
          </Badge>
          <Badge
            variant={timeRange === "7d" ? "default" : "outline"}
            className={`cursor-pointer ${timeRange === "7d" ? "bg-green-400" : "bg-transparent text-gray-400"}`}
            onClick={() => setTimeRange("7d")}
          >
            7d
          </Badge>
        </div>
      </div>

      <div className="flex items-center space-x-1 overflow-x-auto py-2">
        {heatData.map((item, index) => (
          <div key={index} className="flex flex-col items-center pl-1">
            <div className={`w-8 h-8 ${item.color} rounded-full flex items-center justify-center`}>
              {/* You can add icons or text here if needed */}
            </div>
          </div>
        ))}
        <div className="flex flex-col items-center pl-1">
          <div className={`w-8 h-8 bg-green-300 rounded-full flex items-center justify-center`} >
            <span className="text-xs"> Now</span>

          </div>
        </div>
      </div>
    </div>
  );
}
