using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Owned]
public class EResource
{
    [JsonPropertyName("isAvailable")]
    public bool? IsAvailable { get; set; }

    [JsonPropertyName("acsTokenLink")]
    public string? AcsTokenLink { get; set; }
}