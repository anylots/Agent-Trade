"use client";

import { useState, useEffect, useRef, useCallback } from "react";
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Bell, Star, Info, ChevronRight, Copy, Check } from "lucide-react";
import { TokenAvatar } from "@/components/token-avatar";

interface AISignal {
  id: number;
  name: string;
  symbol: string;
  tokenAddress: string;
  logoUri: string;
  priceChange: string;
  price: string;
  volume: string;
  rank: number;
  time: string;
  topPercentage: string;
  avatar: string;
  stats: {
    entryPrice: string;
    marketValue: string;
    volume: string;
    holders: number;
  };
}

interface FetchParams {
  pageNum: number;
  pageSize: number;
  chain: string;
}

// Default page size constant
const DEFAULT_PAGE_SIZE = 12;

export function AISignalsTab() {
  const [signals, setSignals] = useState<AISignal[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [currentPage, setCurrentPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  const [useRealApi, setUseRealApi] = useState(false); // Toggle between mock and real API
  const observer = useRef<IntersectionObserver | null>(null);
  const lastSignalElementRef = useCallback(
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
  const fetchFromApi = async (params: FetchParams): Promise<AISignal[]> => {
    try {
      const response = await fetch(
        `http://localhost:3030/token/ai_signals`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            pageNum: params.pageNum,
            pageSize: params.pageSize,
            extendParam: {
              chain: params.chain
            }
          })
        }
      );
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
  const fetchMockData = async (params: FetchParams): Promise<AISignal[]> => {
    return new Promise((resolve) => {
      setTimeout(async () => {
        try {
          const response = await import("@/data/mockData.json");
          const allData = response.aiSignals;
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
        chain: "solana"
      };

      let data: AISignal[];
      if (useRealApi) {
        data = await fetchFromApi(params);
      } else {
        data = await fetchMockData(params);
      }

      setSignals(data);
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
        chain: "solana"
      };

      let newData: AISignal[];
      if (useRealApi) {
        newData = await fetchFromApi(params);
      } else {
        newData = await fetchMockData(params);
      }

      if (newData.length > 0) {
        setSignals(prev => [...prev, ...newData]);
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

  // Load initial data on component mount
  useEffect(() => {
    loadInitialData();
  }, [useRealApi]);

  // Function to render loading placeholder cards
  const renderLoadingPlaceholders = () => {
    return Array(DEFAULT_PAGE_SIZE).fill(0).map((_, index) => (
      <div key={`placeholder-${index}`}>
        <Card className="bg-slate-900 border-slate-800 overflow-hidden animate-pulse shadow-md">
          <CardHeader className="p-4 pb-2 flex flex-row items-center justify-between">
            <div className="flex items-center space-x-2">
              <div className="h-6 w-16 bg-slate-800 rounded"></div>
              <div className="h-4 w-8 bg-slate-800 rounded"></div>
              <div className="h-4 w-24 bg-slate-800 rounded"></div>
            </div>
            <div className="flex items-center space-x-2">
              <div className="h-4 w-16 bg-slate-800 rounded"></div>
              <div className="h-4 w-4 bg-slate-800 rounded"></div>
            </div>
          </CardHeader>

          <CardContent className="p-4 pt-2">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center space-x-3">
                <div className="h-10 w-10 bg-slate-800 rounded-full"></div>
                <div>
                  <div className="h-5 w-24 bg-slate-800 rounded mb-1"></div>
                  <div className="h-4 w-16 bg-slate-800 rounded"></div>
                </div>
              </div>
              <div className="text-right">
                <div className="h-5 w-16 bg-slate-800 rounded mb-1"></div>
                <div className="h-4 w-20 bg-slate-800 rounded"></div>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4 mb-4">
              <div className="bg-slate-950 p-3 rounded-lg border border-slate-800">
                <div className="h-3 w-12 bg-slate-800 rounded mb-2"></div>
                <div className="grid grid-cols-4 gap-4 mb-1">
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                </div>
                <div className="grid grid-cols-4 gap-4">
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                </div>
              </div>

              <div className="bg-slate-950 p-3 rounded-lg border border-slate-800">
                <div className="h-3 w-12 bg-slate-800 rounded mb-2"></div>
                <div className="grid grid-cols-4 gap-4 mb-1">
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                </div>
                <div className="grid grid-cols-4 gap-4">
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                  <div className="h-3 w-8 bg-slate-800 rounded"></div>
                </div>
              </div>
            </div>

            <div className="bg-slate-950/70 p-3 rounded-lg border border-slate-800/50 mb-4">
              <div className="h-3 w-32 bg-slate-800/70 rounded mb-2"></div>
            </div>
          </CardContent>

          <CardFooter className="p-4 pt-0 grid grid-cols-4 gap-2">
            {Array(4).fill(0).map((_, btnIndex) => (
              <div key={btnIndex} className="h-8 bg-slate-800/70 rounded"></div>
            ))}
          </CardFooter>

          <div className="grid grid-cols-4">
            {Array(4).fill(0).map((_, percentIndex) => (
              <div
                key={percentIndex}
                className={`h-8 ${percentIndex === 0 ? 'bg-indigo-900/20' :
                  percentIndex === 1 ? 'bg-indigo-800/20' :
                    percentIndex === 2 ? 'bg-indigo-700/20' :
                      'bg-indigo-600/20'
                  }`}
              ></div>
            ))}
          </div>
        </Card>
      </div>
    ));
  };

  if (isLoading && signals.length === 0) {
    return (
      <div>
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center space-x-4">
            <Button variant="outline" className="rounded-lg bg-slate-900 border-slate-700 text-slate-200 hover:bg-slate-800 transition-colors">
              <Bell size={16} className="mr-2 text-slate-400" />
              Signal
            </Button>
            <Button variant="outline" className="bg-slate-900 border-slate-700 text-slate-200 hover:bg-slate-800 transition-colors">
              <Star size={16} className="mr-2 text-slate-400" />
              Favorite
            </Button>
          </div>
          <div className="flex items-center space-x-2">
            <Button
              variant="outline"
              className={`mr-2 ${useRealApi ? 'bg-emerald-900/60 border-emerald-800/60' : 'bg-slate-900 border-slate-700'} text-slate-200 hover:bg-slate-800 transition-colors`}
              onClick={() => setUseRealApi(!useRealApi)}
            >
              {useRealApi ? 'Using API' : 'Using SNS Data'}
            </Button>
            <Button variant="outline" className="bg-slate-900 border-slate-700 text-slate-200 hover:bg-slate-800 transition-colors">
              <Info size={16} className="mr-2 text-slate-400" />
              Top10 98.66%
            </Button>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {renderLoadingPlaceholders()}
        </div>
      </div>
    );
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center space-x-4">
          <Button variant="outline" className="bg-slate-900 border-slate-700 text-slate-200 hover:bg-slate-800 transition-colors">
            <Bell size={16} className="mr-2 text-slate-400" />
            Signal
          </Button>
          <Button variant="outline" className="bg-slate-900 border-slate-700 text-slate-200 hover:bg-slate-800 transition-colors">
            <Star size={16} className="mr-2 text-slate-400" />
            Favorite
          </Button>
        </div>
        <div className="flex items-center space-x-2">
          <Button
            variant="outline"
            className={`mr-2 ${useRealApi ? 'bg-emerald-900/60 border-emerald-800/60' : 'bg-slate-900 border-slate-700'} text-slate-200 hover:bg-slate-800 transition-colors`}
            onClick={() => setUseRealApi(!useRealApi)}
          >
            {useRealApi ? 'Using API' : 'Using SNS Data'}
          </Button>
          <Button variant="outline" className="bg-slate-900 border-slate-700 text-slate-200 hover:bg-slate-800 transition-colors">
            <Info size={16} className="mr-2 text-slate-400" />
            Top10 98.66%
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        {signals.map((signal, index) => (
          <div
            key={signal.id}
            ref={index === signals.length - 1 ? lastSignalElementRef : undefined}
          >
            <Card className="bg-slate-900 border-slate-800 overflow-hidden shadow-md hover:shadow-lg transition-all duration-200">
              <CardHeader className="p-4 pb-2 flex flex-row items-center justify-between">
                <div className="flex items-center space-x-2">
                  <Badge className="bg-amber-700/60 text-amber-100 font-medium">{signal.rank}</Badge>
                  {/* <span className="text-gray-400">No Mint</span> */}
                  <Badge variant="outline" className="border-slate-700 text-slate-300">
                    {signal.topPercentage}
                  </Badge>
                </div>
                <div className="flex items-center space-x-2">
                  <Badge variant="outline" className="border-slate-700 text-slate-400">
                    {signal.time}
                  </Badge>
                  <ChevronRight size={16} className="text-slate-500" />
                </div>
              </CardHeader>

              <CardContent className="p-4 pt-1">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center space-x-3">
                    <TokenAvatar className="filter saturate-[60%]" name={signal.name} size="md" />
                    <div>
                      {/* <div className="font-bold">{signal.name}</div> */}
                      <div className="text-sm text-slate-300">{signal.symbol}</div>
                      <div className="text-xs text-slate-500 flex items-center">
                        {signal.tokenAddress.slice(0, 5)}...{signal.tokenAddress.slice(-3)}
                        <button
                          className="ml-1 text-slate-500 hover:text-slate-300 transition-colors"
                          onClick={(e) => {
                            e.stopPropagation();
                            navigator.clipboard.writeText(signal.tokenAddress);
                            const target = e.currentTarget;
                            const copyIcon = target.querySelector('.copy-icon');
                            const checkIcon = target.querySelector('.check-icon');
                            if (copyIcon && checkIcon) {
                              copyIcon.classList.add('hidden');
                              checkIcon.classList.remove('hidden');
                              setTimeout(() => {
                                copyIcon.classList.remove('hidden');
                                checkIcon.classList.add('hidden');
                              }, 2000);
                            }
                          }}
                        >
                          <Copy size={14} className="copy-icon" />
                          <Check size={14} className="check-icon hidden text-emerald-400/80" />
                        </button>
                      </div>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className={`font-bold ${signal.priceChange.startsWith('+') ? 'text-emerald-400/80' : 'text-rose-400/80'}`}>
                      {signal.priceChange}
                    </div>
                    <div className="text-sm text-slate-400">{signal.volume}</div>
                  </div>
                </div>

                <div className="grid grid-cols-1 gap-4 mb-4">
                  {/* <div className="bg-gray-900 p-3 rounded-lg">
                  <div className="text-xs text-gray-400 mb-1">告警</div>
                  <div className="grid grid-cols-4 gap-4">
                    <div className="text-xs text-gray-400">价格</div>
                    <div className="text-xs text-gray-400">市值</div>
                    <div className="text-xs text-gray-400">池子</div>
                    <div className="text-xs text-gray-400">持有人</div>
                  </div>
                  <div className="grid grid-cols-4 gap-4">
                    <div className="text-xs">{signal.stats.entryPrice}</div>
                    <div className="text-xs">{signal.stats.marketValue}</div>
                    <div className="text-xs">{signal.price}</div>
                    <div className="text-xs">{signal.stats.holders}</div>
                  </div>
                </div> */}

                  <div className="bg-slate-950/70 p-3 rounded-lg border border-slate-800/50 backdrop-blur-sm">
                    {/* <div className="text-xs text-gray-400 mb-1">此刻</div> */}
                    <div className="grid grid-cols-4 gap-4">
                      <div className="text-xs text-slate-400">Price</div>
                      <div className="text-xs text-slate-400">TVL</div>
                      <div className="text-xs text-slate-400">24hVolume</div>
                      <div className="text-xs text-slate-400">holders</div>
                    </div>
                    <div className="grid grid-cols-4 gap-4">
                      <div className="text-xs text-rose-300/70">{signal.stats.entryPrice}</div>
                      <div className="text-xs text-indigo-300/70">{signal.stats.marketValue}</div>
                      <div className="text-xs text-indigo-300/70">{signal.price}</div>
                      <div className="text-xs text-indigo-300/70">{signal.stats.holders}</div>
                    </div>
                  </div>
                </div>

                {/* <div className="bg-gray-900 p-3 rounded-lg mb-4">
                  <div className="flex justify-between mb-2">
                    <div className="text-xs text-gray-400">multiple smart wallets have bought</div>
                  </div>
                </div> */}
              </CardContent>

              {/* <CardFooter className="p-4 pt-0 grid grid-cols-4 gap-2">
                {signal.buttons.map((btn, index) => (
                  <Button key={index} variant="outline" className="bg-gray-700 border-gray-600 hover:bg-gray-600">
                    {btn}
                  </Button>
                ))}
              </CardFooter> */}

              {/* <div className="grid grid-cols-4">
                {signal.percentages.map((percentage, index) => (
                  <Button
                    key={index}
                    variant="ghost"
                    className={`rounded-none py-2 ${index === 0 ? 'bg-pink-900/50' :
                      index === 1 ? 'bg-pink-800/50' :
                        index === 2 ? 'bg-pink-700/50' :
                          'bg-pink-600/50'
                      }`}
                  >
                    {percentage}
                  </Button>
                ))}
              </div> */}
            </Card>
          </div>
        ))}
      </div>

      {isLoadingMore && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 mt-4">
          {renderLoadingPlaceholders()}
        </div>
      )}

      {!isLoading && !isLoadingMore && !hasMore && signals.length > 0 && (
        <div className="text-center py-4 mt-4 text-slate-500">
          No more signals to load
        </div>
      )}
    </div>
  );
}
