// src/pages/api/books/[id]/ratings.ts
import type { APIRoute } from 'astro';
import { API_URL } from 'astro:env/server';

export const prerender = false;

export const GET: APIRoute = async ({ params }) => {
  const { id } = params;

  if (!id) {
    return new Response(
      JSON.stringify({ message: 'Identifiant du livre manquant.' }),
      { status: 400, headers: { 'Content-Type': 'application/json' } }
    );
  }

  try {
    const baseUrl = API_URL.endsWith('/api') ? API_URL : `${API_URL}/api`;
    const backendRes = await fetch(`${baseUrl}/books/${id}/ratings`);
    const data = await backendRes.json().catch(() => ({}));

    return new Response(JSON.stringify(data), {
      status: backendRes.status,
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (err: any) {
    return new Response(
      JSON.stringify({ message: err.message || 'Erreur lors de la récupération des notes du livre.' }),
      { status: 500, headers: { 'Content-Type': 'application/json' } }
    );
  }
};

