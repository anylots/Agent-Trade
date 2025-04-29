"use client";

import { useState, useEffect, useRef, useCallback } from "react";
import { Card, CardContent, CardFooter } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { ArrowUpRight, ArrowDownRight, Clock, Users, BarChart3 } from "lucide-react";
import { TokenAvatar } from "@/components/token-avatar";

interface MemeToken {
  id: number;
  name: string;
  description: string;
  price: string;
  volume: string;
  priceChanges: {
    green: string;
    red: string;
    yellow: string;
    blue: string;
    green2: string;
  };
  time: string;
  txs: string;
  holders: string;
  avatar: string;
  category: string;
}

interface FetchParams {
  pageNum: number;
  pageSize: number;
  category?: string;
}

// Default page size constant
const DEFAULT_PAGE_SIZE = 6;

export function MemeTab() {
  const [tokens, setTokens] = useState<MemeToken[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [activeCategory, setActiveCategory] = useState("all");
  const [currentPage, setCurrentPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  const [useRealApi, setUseRealApi] = useState(false); // Toggle between mock and real API
  
  const categories = [
    { id: "all", name: "Pump" },
    { id: "new", name: "Newly Created" },
    { id: "coming", name: "Coming Soon" },
    { id: "open", name: "Already Listed" }
  ];
  const observer = useRef<IntersectionObserver | null>(null);
  const lastTokenElementRef = useCallback(
    (node: HTMLDivElement | null) => {
      if (isLoading || isLoadingMore) return;
      if (observer.current) observer.current.disconnect();
      
      observer.current = new IntersectionObserver((entries) => {
        if (entries[0].isIntersecting && hasMore) {
          loadMoreData();
        }
      });
      
      if (node) observer.current.observe(node);
    },
    [isLoading, isLoadingMore, hasMore]
  );

  // Function to fetch data from API
  const fetchFromApi = async (params: FetchParams): Promise<MemeToken[]> => {
    try {
      const url = `http://localhost:3030/token/meme_tokens`;
      
      // Create request body according to TokenPaginationRequest structure
      const requestBody = {
        pageNum: params.pageNum,
        pageSize: params.pageSize,
        extendParam: undefined as Record<string, string> | undefined
      };
      
      // Add category to extendParam if it's not "all"
      if (params.category && params.category !== "all") {
        requestBody.extendParam = {
          category: params.category
        };
      }
      
      const response = await fetch(url, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(requestBody),
      });
      
      if (!response.ok) {
        throw new Error(`API error: ${response.status}`);
      }
      const data = await response.json();
      return data.list;
    } catch (error) {
      console.error("Error fetching from API:", error);
      throw error;
    }
  };

  // Function to fetch mock data with simulated delay
  const fetchMockData = async (params: FetchParams): Promise<MemeToken[]> => {
    return new Promise((resolve) => {
      setTimeout(async () => {
        try {
          const response = await import("@/data/mockData.json");
          let allData = response.memeTokens;
          
          // Filter by category if needed
          if (params.category && params.category !== "all") {
            const categoryMap: Record<string, string> = {
              "new": "Newly Created",
              "coming": "Coming Soon",
              "open": "Already Listed"
            };
            
            const categoryValue = categoryMap[params.category as keyof typeof categoryMap];
            if (categoryValue) {
              allData = allData.filter(token => token.category === categoryValue);
            }
          }
          
          const startIndex = (params.pageNum - 1) * params.pageSize;
          const endIndex = startIndex + params.pageSize;
          const pageData = allData.slice(startIndex, endIndex);
          
          // If we've reached the end of our mock data
          if (pageData.length === 0) {
            setHasMore(false);
          }
          
          resolve(pageData);
        } catch (error) {
          console.error("Error loading mock data:", error);
          resolve([]);
        }
      }, 800); // Simulate network delay
    });
  };

  // Function to load initial data
  const loadInitialData = async () => {
    setIsLoading(true);
    try {
      const params: FetchParams = {
        pageNum: 1,
        pageSize: DEFAULT_PAGE_SIZE,
        category: activeCategory !== "all" ? activeCategory : undefined
      };
      
      let data: MemeToken[];
      if (useRealApi) {
        data = await fetchFromApi(params);
      } else {
        data = await fetchMockData(params);
      }
      
      setTokens(data);
      setCurrentPage(1);
      setHasMore(data.length === params.pageSize);
    } catch (error) {
      console.error("Error loading initial data:", error);
    } finally {
      setIsLoading(false);
    }
  };

  // Function to load more data
  const loadMoreData = async () => {
    if (isLoadingMore || !hasMore) return;
    
    setIsLoadingMore(true);
    try {
      const nextPage = currentPage + 1;
      const params: FetchParams = {
        pageNum: nextPage,
        pageSize: DEFAULT_PAGE_SIZE,
        category: activeCategory !== "all" ? activeCategory : undefined
      };
      
      let newData: MemeToken[];
      if (useRealApi) {
        newData = await fetchFromApi(params);
      } else {
        newData = await fetchMockData(params);
      }
      
      if (newData.length > 0) {
        setTokens(prev => [...prev, ...newData]);
        setCurrentPage(nextPage);
        setHasMore(newData.length === params.pageSize);
      } else {
        setHasMore(false);
      }
    } catch (error) {
      console.error("Error loading more data:", error);
    } finally {
      setIsLoadingMore(false);
    }
  };

  // Load initial data when component mounts or category/API mode changes
  useEffect(() => {
    loadInitialData();
  }, [activeCategory, useRealApi]);

  // Function to render loading placeholder cards
  const renderLoadingPlaceholders = () => {
    return Array(DEFAULT_PAGE_SIZE).fill(0).map((_, index) => (
      <div key={`placeholder-${index}`}>
        <Card className="bg-gray-800 border-gray-700 overflow-hidden animate-pulse">
          <CardContent className="p-4">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center space-x-3">
                <div className="h-14 w-14 bg-gray-700 rounded-full"></div>
                <div>
                  <div className="h-5 w-24 bg-gray-700 rounded mb-1"></div>
                  <div className="h-4 w-32 bg-gray-700 rounded mb-2"></div>
                  <div className="flex items-center space-x-2">
                    <div className="h-3 w-3 bg-gray-700 rounded-full"></div>
                    <div className="h-3 w-16 bg-gray-700 rounded"></div>
                  </div>
                </div>
              </div>
              <div className="text-right">
                <div className="h-4 w-20 bg-gray-700 rounded mb-1"></div>
                <div className="h-4 w-24 bg-gray-700 rounded mb-2"></div>
                <div className="flex items-center justify-end space-x-1">
                  <div className="h-4 w-12 bg-gray-700 rounded"></div>
                  <div className="h-4 w-12 bg-gray-700 rounded"></div>
                </div>
              </div>
            </div>
            
            <div className="grid grid-cols-5 gap-2">
              {Array(5).fill(0).map((_, i) => (
                <div key={i} className="flex flex-col items-center">
                  <div className="h-4 w-10 bg-gray-700 rounded mb-1"></div>
                  <div className={`h-1 w-full ${
                    i === 0 ? "bg-green-700/30" : 
                    i === 1 ? "bg-red-600/30" : 
                    i === 2 ? "bg-yellow-600/30" : 
                    i === 3 ? "bg-blue-600/30" : 
                    "bg-green-600/30"
                  }`}></div>
                </div>
              ))}
            </div>
          </CardContent>
          
          <CardFooter className="p-0">
            <div className="w-full h-10 bg-green-800/30"></div>
          </CardFooter>
        </Card>
      </div>
    ));
  };

  if (isLoading && tokens.length === 0) {
    return (
      <div>
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center space-x-4 overflow-x-auto pb-2">
            {categories.map(category => (
              <Button
                key={category.id}
                variant={activeCategory === category.id ? "default" : "outline"}
                className={`${
                  activeCategory === category.id 
                    ? "bg-yellow-500 text-black" 
                    : "bg-gray-800 border-gray-700 text-white hover:bg-gray-700"
                }`}
                onClick={() => setActiveCategory(category.id)}
              >
                {category.name}
              </Button>
            ))}
            <Button variant="outline" className="bg-gray-800 border-gray-700 text-white hover:bg-gray-700">
              <Badge className="bg-yellow-500 text-black mr-2">B</Badge>
              Monitoring
            </Button>
          </div>
          <div>
            <Button 
              variant="outline" 
              className={`${useRealApi ? 'bg-green-800 border-green-700' : 'bg-gray-800 border-gray-700'} text-white hover:bg-gray-700`}
              onClick={() => setUseRealApi(!useRealApi)}
            >
              {useRealApi ? 'Using API' : 'Using SNS Data'}
            </Button>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {renderLoadingPlaceholders()}
        </div>
      </div>
    );
  }


  return (
    <div>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center space-x-4 overflow-x-auto pb-2">
          {categories.map(category => (
            <Button
              key={category.id}
              variant={activeCategory === category.id ? "default" : "outline"}
              className={`${
                activeCategory === category.id 
                  ? "bg-yellow-500 text-black" 
                  : "bg-gray-800 border-gray-700 text-white hover:bg-gray-700"
              }`}
              onClick={() => setActiveCategory(category.id)}
            >
              {category.name}
            </Button>
          ))}
          <Button variant="outline" className="bg-gray-800 border-gray-700 text-white hover:bg-gray-700">
            <Badge className="bg-yellow-500 text-black mr-2">B</Badge>
            Monitoring
          </Button>
        </div>
        <div>
          <Button 
            variant="outline" 
            className={`${useRealApi ? 'bg-green-800 border-green-700' : 'bg-gray-800 border-gray-700'} text-white hover:bg-gray-700`}
            onClick={() => setUseRealApi(!useRealApi)}
          >
            {useRealApi ? 'Using API' : 'Using SNS Data'}
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {tokens.map((token, index) => (
          <div 
            key={token.id} 
            ref={index === tokens.length - 1 ? lastTokenElementRef : undefined}
          >
            <Card className="bg-gray-800 border-gray-700 overflow-hidden">
            <CardContent className="p-4">
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center space-x-3">
                  <TokenAvatar name={token.name} size="lg" />
                  <div>
                    <div className="font-bold text-lg">{token.name}</div>
                    <div className="text-sm text-gray-400">{token.description}</div>
                    <div className="flex items-center space-x-2 mt-1">
                      <Clock size={14} className="text-gray-400" />
                      <span className="text-xs text-gray-400">{token.time}</span>
                      {token.category === "Already Listed" && (
                        <Badge className="bg-green-600 text-white text-xs">Run</Badge>
                      )}
                      {token.category === "Coming Soon" && (
                        <Badge className="bg-yellow-500 text-black text-xs">Coming Soon</Badge>
                      )}
                      {(!token.category || (token.category !== "Already Listed" && token.category !== "Coming Soon")) && (
                        <Badge className="bg-gray-600 text-white text-xs">Newly Created</Badge>
                      )}
                    </div>
                  </div>
                </div>
                <div className="text-right">
                  <div className="text-sm text-gray-400">MC: {token.price}</div>
                  <div className="text-sm text-gray-400">VOL: {token.volume}</div>
                  <div className="flex items-center justify-end space-x-1 mt-1">
                    <Badge className="bg-transparent border border-gray-600 text-xs">
                      {token.txs}
                    </Badge>
                    <Badge className="bg-transparent border border-gray-600 text-xs">
                      {token.holders}
                    </Badge>
                  </div>
                </div>
              </div>
              
              <div className="grid grid-cols-5 gap-2">
                <div className="flex flex-col items-center">
                  <div className="text-sm mb-1">{token.priceChanges.green}</div>
                  <div className={`h-1 w-full ${token.priceChanges.green.startsWith("100") ? "bg-green-500" : "bg-green-700"}`}></div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-sm text-red-400 mb-1">{token.priceChanges.red}</div>
                  <div className="h-1 w-full bg-red-600"></div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-sm text-yellow-400 mb-1">{token.priceChanges.yellow}</div>
                  <div className="h-1 w-full bg-yellow-600"></div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-sm text-blue-400 mb-1">{token.priceChanges.blue}</div>
                  <div className="h-1 w-full bg-blue-600"></div>
                </div>
                <div className="flex flex-col items-center">
                  <div className="text-sm text-green-400 mb-1">{token.priceChanges.green2}</div>
                  <div className="h-1 w-full bg-green-600"></div>
                </div>
              </div>
            </CardContent>
            
            <CardFooter className="p-0">
              {/* <Button className="w-full rounded-none bg-green-800 hover:bg-green-700 text-white py-2">
                Buy 0
              </Button> */}
            </CardFooter>
            </Card>
          </div>
        ))}
      </div>
      
      {isLoadingMore && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mt-4">
          {renderLoadingPlaceholders()}
        </div>
      )}
      
      {!isLoading && !isLoadingMore && !hasMore && tokens.length > 0 && (
        <div className="text-center py-4 mt-4 text-gray-400">
          No more tokens to load
        </div>
      )}
    </div>
  );
}
