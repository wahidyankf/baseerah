// Utility
export { cn } from "./utils/cn";

// Primitives — shadcn/radix base layer
// Note: button, badge, card, dialog, sheet primitives are not barrel-exported yet
// (composite counterparts with OSE-specific APIs are still in active use by apps).
// They will replace the composite exports in a later migration phase.
export {
  Command,
  CommandDialog,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandShortcut,
  CommandSeparator,
} from "./primitives/command/command";
export {
  DropdownMenu,
  DropdownMenuPortal,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuLabel,
  DropdownMenuItem,
  DropdownMenuCheckboxItem,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  DropdownMenuSubContent,
} from "./primitives/dropdown-menu/dropdown-menu";
export { Tabs, TabsList, TabsTrigger, TabsContent, tabsListVariants } from "./primitives/tabs/tabs";
export { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider } from "./primitives/tooltip/tooltip";
export { ScrollArea, ScrollBar } from "./primitives/scroll-area/scroll-area";
export { Separator } from "./primitives/separator/separator";

// Components — OSE composites (higher-level, app-specific)
export { Button, buttonVariants } from "./components/button/button";
export { Alert, AlertTitle, AlertDescription, alertVariants } from "./components/alert/alert";
export { Input } from "./components/input/input";
export { Card, CardHeader, CardFooter, CardTitle, CardDescription, CardContent } from "./components/card/card";
export { Label } from "./components/label/label";
export {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
} from "./components/dialog/dialog";
export { Icon, type IconName, type IconProps } from "./components/icon/icon";
export { Toggle, type ToggleProps } from "./components/toggle/toggle";
export { ProgressRing, type ProgressRingProps } from "./components/progress-ring/progress-ring";
export { Sheet, type SheetProps } from "./components/sheet/sheet";
export { AppHeader, type AppHeaderProps } from "./components/app-header/app-header";
export { HuePicker, HUES } from "./components/hue-picker/hue-picker";
export type { HueName, HuePickerProps } from "./components/hue-picker/hue-picker";
export { InfoTip } from "./components/info-tip/info-tip";
export type { InfoTipProps } from "./components/info-tip/info-tip";
export { StatCard } from "./components/stat-card/stat-card";
export type { StatCardProps } from "./components/stat-card/stat-card";
export { TabBar } from "./components/tab-bar/tab-bar";
export type { TabItem, TabBarProps } from "./components/tab-bar/tab-bar";
export { SideNav } from "./components/side-nav/side-nav";
export type { SideNavBrand, SideNavProps } from "./components/side-nav/side-nav";
export { HighlightText, highlightText } from "./components/highlight-text/highlight-text";
export { default as ScrollToTop } from "./components/scroll-to-top/scroll-to-top";
export { SearchComponent } from "./components/search-component/search-component";
export { default as ThemeToggle } from "./components/theme-toggle/theme-toggle";
export { Textarea } from "./components/textarea/textarea";
export { Badge, badgeVariants } from "./components/badge/badge";
export type { BadgeProps } from "./components/badge/badge";
