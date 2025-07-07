import React, { useState } from "react";
import { minecraftLogin } from "@/lib/commands/minecraft";
import { invoke } from '@tauri-apps/api/core';

export const MinecraftLoginForm: React.FC = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [twofa, setTwofa] = useState("");
  const [show2fa, setShow2fa] = useState(false);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");
  const [loading, setLoading] = useState(false);
  const [serverUrl, setServerUrl] = useState("https://account.ely.by/api/authlib-injector");

  const handleLogin = async () => {
    setLoading(true);
    setError("");
    setSuccess("");
    const result = await minecraftLogin(username, password, twofa);
    setLoading(false);

    if (result?.error === "2fa") {
      setShow2fa(true);
      setError("Two-factor authentication required. Enter your code.");
    } else if (result?.error) {
      setError(result.errorMessage || "Login failed.");
    } else {
      setError("");
      setShow2fa(false);
      setSuccess("Login successful!");
      if (result.accessToken && result.clientToken && result.selectedProfile) {
        localStorage.setItem("elyby_accessToken", result.accessToken);
        localStorage.setItem("elyby_clientToken", result.clientToken);
        // Update backend profile with Ely.by info
        const profile = result.selectedProfile;
        await invoke('set_elyby_profile', {
          username: profile.name,
          uuid: profile.id,
          accessToken: result.accessToken
        });
        // Notify sidebar/profile to refresh
        window.dispatchEvent(new Event('elyby-profile-updated'));
      }
      console.log("Minecraft login result:", result);
    }
  };

  return (
    <div className="flex flex-col gap-4 w-full max-w-md mx-auto bg-[#23252b] p-8 rounded-xl shadow-lg border border-[#2D2F32]">
      <h2 className="text-2xl font-bold text-white mb-2">Minecraft Login (Ely.by)</h2>
      <input
        type="text"
        placeholder="Username or Email"
        value={username}
        onChange={e => setUsername(e.target.value)}
        className="bg-[#181a20] border border-[#2D2F32] rounded px-4 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-sky-500"
      />
      <input
        type="password"
        placeholder="Password"
        value={password}
        onChange={e => setPassword(e.target.value)}
        className="bg-[#181a20] border border-[#2D2F32] rounded px-4 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-sky-500"
      />
      {show2fa && (
        <input
          type="text"
          placeholder="2FA Code"
          value={twofa}
          onChange={e => setTwofa(e.target.value)}
          className="bg-[#181a20] border border-[#2D2F32] rounded px-4 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-sky-500"
        />
      )}
      <input
        type="text"
        placeholder="Ely.by Server URL"
        value={serverUrl}
        onChange={e => setServerUrl(e.target.value)}
        className="bg-[#181a20] border border-[#2D2F32] rounded px-4 py-2 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-sky-500"
      />
      <button
        onClick={handleLogin}
        disabled={loading}
        className="bg-sky-500 hover:bg-sky-600 text-white font-bold py-2 rounded transition disabled:opacity-50"
      >
        {loading ? "Logging in..." : "Login"}
      </button>
      {error && <div className="text-red-500 font-semibold">{error}</div>}
      {success && <div className="text-green-500 font-semibold">{success}</div>}
    </div>
  );
}; 