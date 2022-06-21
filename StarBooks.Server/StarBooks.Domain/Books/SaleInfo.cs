using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Table("saleInfos"), Owned]
public record SaleInfo(
    [property: JsonIgnore, Key] Guid Id,
    [property: JsonPropertyName("country")] string Country,
    [property: JsonPropertyName("isEbook")] bool IsEbook,
    [property: JsonPropertyName("listPrice")] Price ListPrice,
    [property: JsonPropertyName("retailPrice")] Price RetailPrice
);