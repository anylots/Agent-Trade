import Image from "next/image";
import Link from "next/link";
import { Search } from "lucide-react";
import { Button } from "@/components/ui/button";

export function Header() {
  return (
    <header className="bg-black text-white p-4 flex items-center justify-between">
      <div className="flex items-center space-x-6">
        <Link href="/" className="flex items-center">
          <div className="text-green-400 text-2xl font-bold">TrendAgent</div>
        </Link>
        <nav className="hidden md:flex space-x-6">
          <Link href="/" className="hover:text-yellow-400">Favorite</Link>
          <Link href="/ai-signals" className="hover:text-yellow-400">AI Signal</Link>
          <Link href="/meme" className="hover:text-yellow-400">Meme</Link>
          <Link href="#" className="hover:text-yellow-400">New launch</Link>
          <Link href="#" className="hover:text-yellow-400">Popular</Link>
          <Link href="#" className="hover:text-yellow-400">AI Monitoring</Link>
          <Link href="#" className="hover:text-yellow-400">Trading</Link>
        </nav>
      </div>

      <div className="flex items-center space-x-4">
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400" size={16} />
          <input
            type="text"
            placeholder="Search Token/Wallet"
            className="bg-gray-800 rounded-full pl-10 pr-4 py-1 text-sm w-52 focus:outline-none focus:ring-1 focus:ring-green-400"
          />
          {/* <span className="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 text-xs">Ctrl+K</span> */}
        </div>

        <div className="flex items-center space-x-2">
          <Button variant="outline" className="bg-transparent border-gray-700 text-white hover:bg-gray-800 rounded-full">
            <span className="mr-1 text-white">Solana</span>
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="lucide lucide-chevron-down text-white"><path d="m6 9 6 6 6-6" /></svg>
          </Button>

          <Button className="bg-green-400 hover:bg-green-500 text-black rounded-full">
            Log In
          </Button>
        </div>
      </div>
    </header>
  );
}
