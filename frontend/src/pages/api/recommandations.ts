// src/pages/api/recommandations.ts
import type { APIRoute } from 'astro';
import { API_URL } from 'astro:env/server';

export const prerender = false;

export const GET: APIRoute = async ({ cookies }) => {
  const token = cookies.get('auth_token')?.value;

  if (!token) {
    return new Response(
      JSON.stringify({ erreur: 'Vous devez être connecté pour obtenir des recommandations.' }),
      {
        status: 401,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }

  try {
    const baseUrl = API_URL.endsWith('/api') ? API_URL : `${API_URL}/api`;
    const backendRes = await fetch(`${baseUrl}/recommandations`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });

    const data = await backendRes.json().catch(() => ([]));

    return new Response(JSON.stringify(data), {
      status: backendRes.status,
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (error: any) {
    return new Response(
      JSON.stringify({ erreur: error.message || 'Impossible de contacter le serveur backend.' }),
      {
        status: 500,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }
};

