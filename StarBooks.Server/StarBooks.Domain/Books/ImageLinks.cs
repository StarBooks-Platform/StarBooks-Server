using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Owned]
public record ImageLinks(
    [property: JsonPropertyName("thumbnail")] string ThumbnailUrl
);
