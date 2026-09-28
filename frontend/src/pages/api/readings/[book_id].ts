import type { APIRoute } from 'astro';
import { API_URL } from 'astro:env/server';

export const prerender = false;

export const DELETE: APIRoute = async ({ params, cookies }) => {
  const token = cookies.get('auth_token')?.value;

  if (!token) {
    console.warn('[API DELETE /api/readings/[book_id]] Aucun auth_token trouvé dans les cookies');
    return new Response(
      JSON.stringify({ erreur: 'Vous devez être connecté pour supprimer une note.' }),
      {
        status: 401,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }

  const { book_id } = params;
  if (!book_id) {
    return new Response(
      JSON.stringify({ erreur: 'Identifiant du livre manquant.' }),
      {
        status: 400,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }

  try {
    const baseUrl = API_URL.endsWith('/api') ? API_URL : `${API_URL}/api`;
    const endpoint = `${baseUrl}/readings/${encodeURIComponent(book_id)}`;

    console.log(`[API DELETE /api/readings/[book_id]] Relais DELETE vers ${endpoint}`);

    const response = await fetch(endpoint, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`,
      },
    });

    console.log(`[API DELETE /api/readings/[book_id]] Réponse du backend statut : ${response.status}`);

    let data: any;
    const contentType = response.headers.get('content-type');
    if (contentType && contentType.includes('application/json')) {
      data = await response.json().catch(() => ({}));
    } else {
      const text = await response.text();
      data = { erreur: text || `Erreur serveur (${response.status})` };
    }

    return new Response(JSON.stringify(data), {
      status: response.status,
      headers: { 'Content-Type': 'application/json' },
    });
  } catch (error: any) {
    console.error('[API DELETE /api/readings/[book_id]] Erreur lors du relais :', error);
    return new Response(
      JSON.stringify({ erreur: error.message || 'Impossible de contacter le serveur backend.' }),
      {
        status: 500,
        headers: { 'Content-Type': 'application/json' },
      }
    );
  }
};

