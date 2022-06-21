using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Table("searchInfos"), Owned]
public record SearchInfo(
    [property: JsonIgnore, Key] Guid Id,
    [property: JsonPropertyName("textSnippet")] string TextSnippet
);
