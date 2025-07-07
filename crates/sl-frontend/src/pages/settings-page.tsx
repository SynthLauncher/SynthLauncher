import { MinecraftLoginForm } from "@/components/minecraft-login-form";

export default function SettingsPage() {
	return (
		<div className="p-8">
			{/* ...existing settings UI... */}
			<h2 className="text-xl font-bold mt-8 mb-4">Minecraft Login (Ely.by)</h2>
			<p className="text-sm text-gray-500 mb-4">
				This is for Ely.by Authlib-Injector accounts only.
			</p>
			<MinecraftLoginForm />
		</div>
	);
}
