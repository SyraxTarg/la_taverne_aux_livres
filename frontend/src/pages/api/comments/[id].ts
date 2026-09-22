// src/pages/api/comments/[id].ts
import type { APIRoute } from 'astro';
import { API_URL } from 'astro:env/server';

export const prerender = false;

export const PUT: APIRoute = async ({ params, request, cookies }) => {
  const token = cookies.get('auth_token')?.value;

  if (!token) {
    return new Response(
      JSON.stringify({ message: 'Vous devez être connecté pour modifier un commentaire.' }),
      { status: 401, headers: { 'Content-Type': 'application/json' } }
    );
  }

  const { id } = params;
  if (!id) {
    return new Response(
      JSON.stringify({ message: 'Identifiant du commentaire manquant.' }),
      { status: 400, headers: { 'Content-Type': 'application/json' } }
    );
  }

  try {
    const body = await request.json();
    const { content } = body;

    if (!content || !content.trim()) {
      return new Response(
        JSON.stringify({ message: 'Le contenu du commentaire ne peut pas être vide.' }),
        { status: 400, headers: { 'Content-Type': 'application/json' } }
      );
    }

    const baseUrl = API_URL.endsWith('/api') ? API_URL : `${API_URL}/api`;
    const backendRes = await fetch(`${baseUrl}/comments/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
      body: JSON.stringify({ content: content.trim() }),
    });

    const data = await backendRes.json().catch(() => ({}));

    return new Response(JSON.stringify(data), {
      status: backendRes.status,
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (err: any) {
    return new Response(
      JSON.stringify({ message: err.message || 'Erreur lors de la modification du commentaire.' }),
      { status: 500, headers: { 'Content-Type': 'application/json' } }
    );
  }
};

export const DELETE: APIRoute = async ({ params, cookies }) => {
  const token = cookies.get('auth_token')?.value;

  if (!token) {
    return new Response(
      JSON.stringify({ message: 'Vous devez être connecté pour supprimer un commentaire.' }),
      { status: 401, headers: { 'Content-Type': 'application/json' } }
    );
  }

  const { id } = params;
  if (!id) {
    return new Response(
      JSON.stringify({ message: 'Identifiant du commentaire manquant.' }),
      { status: 400, headers: { 'Content-Type': 'application/json' } }
    );
  }

  try {
    const baseUrl = API_URL.endsWith('/api') ? API_URL : `${API_URL}/api`;
    const backendRes = await fetch(`${baseUrl}/comments/${id}`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });

    const data = await backendRes.json().catch(() => ({}));

    return new Response(JSON.stringify(data), {
      status: backendRes.status,
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (err: any) {
    return new Response(
      JSON.stringify({ message: err.message || 'Erreur lors de la suppression du commentaire.' }),
      { status: 500, headers: { 'Content-Type': 'application/json' } }
    );
  }
};

