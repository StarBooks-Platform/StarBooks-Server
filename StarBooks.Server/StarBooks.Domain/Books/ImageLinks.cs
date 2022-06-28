using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Owned]
public class ImageLinks
{
    [JsonPropertyName("thumbnail")]
    public string? ThumbnailUrl { get; set; }
}
