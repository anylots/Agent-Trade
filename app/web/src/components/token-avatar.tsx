"use client";

import { Avatar, AvatarFallback } from "@/components/ui/avatar";

interface TokenAvatarProps {
  name: string;
  size?: "sm" | "md" | "lg";
  className?: string;
}

export function TokenAvatar({ name, size = "md", className = "" }: TokenAvatarProps) {
  // Generate a consistent color based on the name
  const getColorFromName = (name: string) => {
    const colors = [
      "bg-red-500",
      "bg-yellow-500",
      "bg-green-500",
      "bg-blue-500",
      "bg-indigo-500",
      "bg-purple-500",
      "bg-pink-500",
      "bg-orange-500",
      "bg-teal-500",
      "bg-cyan-500"
    ];
    
    let hash = 0;
    for (let i = 0; i < name.length; i++) {
      hash = name.charCodeAt(i) + ((hash << 5) - hash);
    }
    
    return colors[Math.abs(hash) % colors.length];
  };

  const getInitials = (name: string) => {
    return name.charAt(0).toUpperCase();
  };

  const sizeClasses = {
    sm: "h-8 w-8",
    md: "h-10 w-10",
    lg: "h-14 w-14"
  };

  const fontSizeClasses = {
    sm: "text-sm",
    md: "text-base",
    lg: "text-xl"
  };

  return (
    <Avatar className={`${sizeClasses[size]} ${getColorFromName(name)} ${className}`}>
      <AvatarFallback className={`${getColorFromName(name)} text-white font-bold ${fontSizeClasses[size]}`}>
        {getInitials(name)}
      </AvatarFallback>
    </Avatar>
  );
}
