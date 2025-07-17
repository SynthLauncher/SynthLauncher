import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { OctagonAlertIcon } from "lucide-react";

export default function AlertWithBackgroundDemo() {
  return (
    <Alert
      className="bg-destructive text-destructive-foreground [&>svg]:text-destructive-foreground"
    >
      <OctagonAlertIcon className="h-4 w-4" />
      <AlertTitle>Something Went Wrong</AlertTitle>
      <AlertDescription>
        An error occurred while processing your request.
      </AlertDescription>
    </Alert>
  );
}
