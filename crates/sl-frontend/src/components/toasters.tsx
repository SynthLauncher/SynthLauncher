import { CheckCheckIcon, InfoIcon } from 'lucide-react';
import { toast } from 'sonner';

export const ToastSuccess = (description: string) => {
	toast.success(
		<div className="flex flex-col gap-1">
			<span className="font-semibold leading-tight">
				Operation completed successfully
			</span>
			<span className="text-sm leading-snug">{description}</span>
		</div>,
		{
			icon: <CheckCheckIcon size={20} className="text-green-600" />,
			style: {
				border: '2px solid rgba(22, 163, 74)',
				color: 'rgb(22, 163, 74)',
				fontSize: '0.875rem',
				fontWeight: 400,
				padding: '12px 16px',
				display: 'flex',
				alignItems: 'center',
				gap: '12px',
				borderRadius: '0.5rem',
				backgroundColor:
					'color-mix(in oklab, light-dark(var(--color-green-600), var(--color-green-400)) 10%, var(--background))',
			} as React.CSSProperties,
		}
	);
};

export const ToastInfo = (description: string) => {
	toast.info(
		<div className="flex flex-col gap-1">
			<span className="font-semibold leading-tight">Important Information</span>
			<span className="text-sm leading-snug">{description}</span>
		</div>,
		{
			icon: <InfoIcon size={20} className="text-cyan-600" />,
			style: {
				border: '2px solid rgba(8, 145, 178, 0.5)',
				color: 'rgb(8, 145, 178)',
				fontSize: '0.875rem',
				fontWeight: 400,
				padding: '12px 16px',
				display: 'flex',
				alignItems: 'center',
				gap: '12px',
				borderRadius: '0.5rem',
				backgroundColor:
					'color-mix(in oklab, light-dark(var(--color-sky-600), var(--color-sky-400)) 10%, var(--background))',
			} as React.CSSProperties,
		}
	);
};
