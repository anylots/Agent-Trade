import { Sun, MessageCircle, LifeBuoy, Settings, Bot, ArrowRight, Zap } from "lucide-react";

export function Footer() {
  return (
    <footer className="fixed bottom-0 left-0 right-0 bg-black border-t border-gray-800 py-2 px-4">
      <div className="container mx-auto flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <div className="flex items-center text-yellow-400">
            <Sun size={16} className="mr-1" />
            <span className="text-xs">Invitation</span>
          </div>
          <div className="flex items-center text-gray-400">
            <MessageCircle size={16} className="mr-1" />
            <span className="text-xs">Member Subscription</span>
          </div>
        </div>

        <div className="flex items-center space-x-4">
          <div className="flex items-center text-gray-400">
            <LifeBuoy size={16} className="mr-1" />
            <span className="text-xs">Online Customer Service</span>
          </div>
          <div className="flex items-center text-gray-400">
            <Settings size={16} className="mr-1" />
            <span className="text-xs">Feedback</span>
          </div>
          <div className="flex items-center text-gray-400">
            <Bot size={16} className="mr-1" />
            <span className="text-xs">Tutorial</span>
          </div>
          <div className="flex items-center text-gray-400">
            <ArrowRight size={16} className="mr-1" />
            <span className="text-xs">TG Bot</span>
          </div>
          <div className="flex items-center text-gray-400">
            <Zap size={16} className="mr-1" />
            <span className="text-xs">APP Download</span>
          </div>
        </div>

        <div className="text-gray-400 text-xs">
          SOL: $149.34
        </div>
      </div>
    </footer>
  );
}
