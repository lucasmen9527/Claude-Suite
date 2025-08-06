import * as React from "react";
import { motion, AnimatePresence } from "framer-motion";
import { createPortal } from "react-dom";
import { cn } from "@/lib/utils";

interface PopoverProps {
  /**
   * The trigger element
   */
  trigger: React.ReactNode;
  /**
   * The content to display in the popover
   */
  content: React.ReactNode;
  /**
   * Whether the popover is open
   */
  open?: boolean;
  /**
   * Callback when the open state changes
   */
  onOpenChange?: (open: boolean) => void;
  /**
   * Optional className for the content
   */
  className?: string;
  /**
   * Alignment of the popover relative to the trigger
   */
  align?: "start" | "center" | "end";
  /**
   * Side of the trigger to display the popover
   */
  side?: "top" | "bottom";
  /**
   * Whether to render as a modal (using portal)
   */
  modal?: boolean;
}

/**
 * Popover component for displaying floating content
 * 
 * @example
 * <Popover
 *   trigger={<Button>Click me</Button>}
 *   content={<div>Popover content</div>}
 *   side="top"
 * />
 */
export const Popover: React.FC<PopoverProps> = ({
  trigger,
  content,
  open: controlledOpen,
  onOpenChange,
  className,
  align = "center",
  side = "bottom",
  modal = false,
}) => {
  const [internalOpen, setInternalOpen] = React.useState(false);
  const open = controlledOpen !== undefined ? controlledOpen : internalOpen;
  const setOpen = onOpenChange || setInternalOpen;
  
  const triggerRef = React.useRef<HTMLDivElement>(null);
  const contentRef = React.useRef<HTMLDivElement>(null);
  const [position, setPosition] = React.useState({ top: 0, left: 0 });
  
  // Calculate position for modal popovers
  React.useEffect(() => {
    if (modal && open && triggerRef.current) {
      const rect = triggerRef.current.getBoundingClientRect();
      let top = side === "top" ? rect.top - 8 : rect.bottom + 8;
      let left = rect.left;
      
      // Adjust based on alignment
      if (align === "center") {
        left = rect.left + rect.width / 2;
      } else if (align === "end") {
        left = rect.right;
      }
      
      setPosition({ top, left });
    }
  }, [modal, open, side, align]);
  
  // Close on click outside
  React.useEffect(() => {
    if (!open) return;
    
    const handleClickOutside = (event: MouseEvent) => {
      if (
        triggerRef.current &&
        contentRef.current &&
        !triggerRef.current.contains(event.target as Node) &&
        !contentRef.current.contains(event.target as Node)
      ) {
        setOpen(false);
      }
    };
    
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [open, setOpen]);
  
  // Close on escape
  React.useEffect(() => {
    if (!open) return;
    
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setOpen(false);
      }
    };
    
    document.addEventListener("keydown", handleEscape);
    return () => document.removeEventListener("keydown", handleEscape);
  }, [open, setOpen]);
  
  const alignClass = {
    start: "left-0",
    center: "left-1/2 -translate-x-1/2",
    end: "right-0",
  }[align];
  
  const sideClass = side === "top" ? "bottom-full mb-2" : "top-full mt-2";
  const animationY = side === "top" ? { initial: 10, exit: 10 } : { initial: -10, exit: -10 };
  
  const popoverContent = (
    <AnimatePresence>
      {open && (
        <motion.div
          ref={contentRef}
          initial={{ opacity: 0, scale: 0.95, y: animationY.initial }}
          animate={{ opacity: 1, scale: 1, y: 0 }}
          exit={{ opacity: 0, scale: 0.95, y: animationY.exit }}
          transition={{ duration: 0.15 }}
          className={cn(
            "min-w-[200px] rounded-md border border-border bg-popover dark:!bg-gray-900 dark:!border-gray-700 p-4 text-popover-foreground dark:!text-gray-100 shadow-md",
            modal ? "absolute z-layer-status-modal" : cn("absolute z-50", sideClass, alignClass),
            modal && align === "center" && "-translate-x-1/2",
            modal && align === "end" && "-translate-x-full",
            className
          )}
          style={modal ? { top: position.top, left: position.left } : undefined}
        >
          {content}
        </motion.div>
      )}
    </AnimatePresence>
  );
  
  return (
    <div className="relative inline-block">
      <div
        ref={triggerRef}
        onClick={() => setOpen(!open)}
      >
        {trigger}
      </div>
      
      {modal ? createPortal(popoverContent, document.body) : popoverContent}
    </div>
  );
}; 