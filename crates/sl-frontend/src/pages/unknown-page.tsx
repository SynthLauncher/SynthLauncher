import { Button } from '@/components/ui/button';
import { ArrowLeft } from 'lucide-react';

export const UnknownPage = () => {
	return (
		<div className="flex items-center justify-center h-full p-8 text-white">
			<div className="text-center max-w-md">
				<div className="mb-6 text-6xl">🚧</div>

				<h2 className="text-3xl font-bold mb-3">Page Not Found</h2>

				<p className="text-gray-400 mb-6">
					Oops! This page doesn't exist. You might want to head back to the home
					page.
				</p>

				<Button
					// onClick={goHome}
					// disabled={redirecting}
					variant="error"
					className="inline-flex items-center gap-2"
				>
					<ArrowLeft className="w-4 h-4" />
					{/* {redirecting ? 'Redirecting...' : 'Go to Home'} */}
				</Button>
			</div>
		</div>
	);
};
