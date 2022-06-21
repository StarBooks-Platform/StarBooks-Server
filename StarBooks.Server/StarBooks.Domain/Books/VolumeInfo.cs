using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Table("volumeInfos"), Owned]
public record VolumeInfo(
    [property: JsonIgnore, Key] Guid Id,
    [property: JsonPropertyName("title")] string Title,
    [property: JsonPropertyName("authors")] List<string> Authors,
    [property: JsonPropertyName("publisher")] string Publisher,
    [property: JsonPropertyName("publishedDate")] string PublishedDate,
    [property: JsonPropertyName("description")] string Description,
    [property: JsonPropertyName("industryIdentifiers")] List<IndustryIdentifier> Identifiers,
    [property: JsonPropertyName("pageCount")] int PageCount,
    [property: JsonPropertyName("printType")] string PrintType,
    [property: JsonPropertyName("categories")] List<string> Categories,
    [property: JsonPropertyName("averageRating")] decimal AverageRating,
    [property: JsonPropertyName("ratingsCount")] int RatingsCount,
    [property: JsonPropertyName("imageLinks")] ImageLinks ImageLinks,
    [property: JsonPropertyName("language")] string Language,
    [property: JsonPropertyName("previewLink")] string PreviewLink,
    [property: JsonPropertyName("InfoLink")] string InfoLink
);
