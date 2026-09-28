// src/utils/html.ts

/**
 * Liste des balises HTML courantes standardisées.
 */
export const HTML_TAGS: readonly string[] = [
  'a', 'abbr', 'address', 'area', 'article', 'aside', 'audio', 'b', 'base', 'bdi',
  'bdo', 'blockquote', 'body', 'br', 'button', 'canvas', 'caption', 'cite', 'code',
  'col', 'colgroup', 'data', 'datalist', 'dd', 'del', 'details', 'dfn', 'dialog',
  'div', 'dl', 'dt', 'em', 'embed', 'fieldset', 'figcaption', 'figure', 'footer',
  'form', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'head', 'header', 'hgroup', 'hr',
  'html', 'i', 'iframe', 'img', 'input', 'ins', 'kbd', 'label', 'legend', 'li',
  'link', 'main', 'map', 'mark', 'menu', 'meta', 'meter', 'nav', 'noscript',
  'object', 'ol', 'optgroup', 'option', 'output', 'p', 'param', 'picture', 'pre',
  'progress', 'q', 'rp', 'rt', 'ruby', 's', 'samp', 'script', 'section', 'select',
  'small', 'source', 'span', 'strong', 'style', 'sub', 'summary', 'sup', 'table',
  'tbody', 'td', 'template', 'textarea', 'tfoot', 'th', 'thead', 'time', 'title',
  'tr', 'track', 'u', 'ul', 'var', 'video', 'wbr'
] as const;

/**
 * Supprime toutes les balises HTML d'un texte tout en conservant une mise en page aérée.
 * 
 * @param text Le texte source pouvant contenir des balises HTML.
 * @returns Le texte nettoyé de toute balise HTML.
 */
export function removeHtmlTags(text?: string | null): string {
  if (!text) return '';

  // Conversion des balises de saut de ligne et de fin de paragraphe en retours à la ligne
  let cleaned = text
    .replace(/<br\s*\/?>/gi, '\n')
    .replace(/<\/p>/gi, '\n\n')
    .replace(/<\/(div|li|tr|h[1-6])>/gi, '\n');

  // Expression régulière basée sur la liste des balises HTML autorisées
  const tagsRegex = new RegExp(`</?(?:${HTML_TAGS.join('|')})\\b[^>]*>`, 'gi');
  cleaned = cleaned.replace(tagsRegex, '');

  // Sécurité supplémentaire : suppression de toute balise HTML restante
  cleaned = cleaned.replace(/<[^>]+>/g, '');

  // Décodage des entités HTML usuelles
  cleaned = cleaned
    .replace(/&nbsp;/g, ' ')
    .replace(/&amp;/g, '&')
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&apos;/g, "'")
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>');

  // Nettoyage des espaces et sauts de ligne superflus
  return cleaned.replace(/\n{3,}/g, '\n\n').trim();
}

