using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;

namespace StarBooks.Domain.Books;

[Table("books")]
public record BookModel(
    [property: JsonIgnore] [property: Key] Guid Id,
    [property: JsonPropertyName("volumeInfo")] VolumeInfo VolumeInfo,
    [property: JsonPropertyName("saleInfo")] SaleInfo SaleInfo,
    [property: JsonPropertyName("accessInfo")] AccessInfo AccessInfo,
    [property: JsonPropertyName("searchInfo")] SearchInfo SearchInfo
);
