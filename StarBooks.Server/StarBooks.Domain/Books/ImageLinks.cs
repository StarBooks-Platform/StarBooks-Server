using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Table("imageLinks"), Owned]
public record ImageLinks(
    [property: JsonIgnore, Key] Guid Id,
    [property: JsonPropertyName("thumbnail")] string ThumbnailUrl
);
